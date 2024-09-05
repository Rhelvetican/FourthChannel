mod threadworks;

use std::{
    fmt::Debug,
    mem::ManuallyDrop,
    path::PathBuf,
    sync::Arc,
    thread::{Builder as ThreadBuilder, JoinHandle},
};

use self::threadworks::DatabaseThreadWork;

use rusqlite::Connection;
use tokio::{
    runtime::Handle,
    select,
    sync::{
        mpsc::{channel, Receiver, Sender},
        oneshot::channel as onechan,
        Notify,
    },
};

use crate::utils::Result;

#[derive(Debug)]
pub enum DatabaseProvider {
    InMem,
    InFile(PathBuf),
}

impl DatabaseProvider {
    pub fn connect(self) -> Result<Connection> {
        Ok(match self {
            DatabaseProvider::InMem => Connection::open_in_memory()?,
            DatabaseProvider::InFile(path) => Connection::open(path)?,
        })
    }
}

pub struct Database {
    inner: Arc<DbInner>,
}

impl Database {
    pub fn with_provider(provider: DatabaseProvider) -> Result<Self> {
        let conn = provider.connect()?;
        let (work_tx, work_rx) = channel(10);
        let shutdown_notify = Arc::new(Notify::new());

        let handle = Handle::current();
        let thread = DbThread::new(conn, handle, work_rx, shutdown_notify.clone());
        let join_handle = ManuallyDrop::new(thread.start());

        Ok(Self {
            inner: Arc::new(DbInner {
                handle: join_handle,
                work_tx,
                shutdown_notify,
            }),
        })
    }

    pub async fn enqueue<F: FnOnce(&mut Connection) + Send + 'static>(&self, f: F) -> Result<()> {
        let tw = AnyDbTw::new(f);

        self.inner
            .work_tx
            .send(Box::new(tw))
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn query<
        F: FnOnce(&mut Connection) -> R + Send + 'static,
        R: Send + Debug + 'static,
    >(
        &self,
        f: F,
    ) -> Result<()> {
        let (tx, rx) = onechan();

        self.enqueue(move |conn| {
            let r = f(conn);
            tx.send(r).unwrap();
        })
        .await?;

        rx.await.map_err(|e| e.to_string())?;

        Ok(())
    }
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

struct DbInner {
    handle: ManuallyDrop<JoinHandle<()>>,
    work_tx: Sender<Box<dyn DatabaseThreadWork>>,
    shutdown_notify: Arc<Notify>,
}

impl Drop for DbInner {
    fn drop(&mut self) {
        self.shutdown_notify.notify_one();
        let handle = unsafe { ManuallyDrop::take(&mut self.handle) };
        handle.join().unwrap();
    }
}

struct DbThread {
    conn: Connection,
    rt_handle: Handle,
    work_rx: Receiver<Box<dyn DatabaseThreadWork>>,
    shutdown_notify: Arc<Notify>,
    shutdown: bool,
}

impl DbThread {
    fn new(
        conn: Connection,
        rt_handle: Handle,
        work_rx: Receiver<Box<dyn DatabaseThreadWork>>,
        shutdown_notify: Arc<Notify>,
    ) -> Self {
        Self {
            conn,
            rt_handle,
            work_rx,
            shutdown_notify,
            shutdown: false,
        }
    }

    fn start(self) -> JoinHandle<()> {
        ThreadBuilder::new()
            .name(String::from("Database Thread"))
            .spawn(move || {
                let mut thread = self;
                let handle = thread.rt_handle.clone();
                handle.block_on(async move {
                    thread.run_loop().await;
                });
            })
            .unwrap()
    }

    async fn run_loop(&mut self) {
        while !self.shutdown {
            self.poll_once().await;
        }
    }

    async fn poll_once(&mut self) {
        select! {
            _ = self.shutdown_notify.notified() => {
                self.shutdown = true;
            },
            maybe_work = self.work_rx.recv() => {
                if let Some(mut work) = maybe_work {
                    work.perform(&mut self.conn);
                } else {
                    // No more work to perform, the thread is requested to terminate.
                    self.shutdown = true;
                }
            }
        };
    }
}

struct AnyDbTw<F>
where
    F: FnOnce(&mut Connection) + Send,
{
    f: Option<F>,
}

impl<F> AnyDbTw<F>
where
    F: FnOnce(&mut Connection) + Send,
{
    fn new(f: F) -> Self {
        Self { f: Some(f) }
    }
}

impl<F> DatabaseThreadWork for AnyDbTw<F>
where
    F: FnOnce(&mut Connection) + Send,
{
    fn perform(&mut self, conn: &mut Connection) {
        let f = self.f.take().unwrap();
        f(conn)
    }
}
