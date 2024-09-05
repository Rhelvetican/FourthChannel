use rusqlite::Connection;

pub(crate) trait DatabaseThreadWork: Send {
    fn perform(&mut self, conn: &mut Connection);
}

pub(super) struct AnyDatabaseThreadWork<F>
where
    F: FnOnce(&mut Connection) + Send,
{
    f: Option<F>,
}

impl<F> AnyDatabaseThreadWork<F>
where
    F: FnOnce(&mut Connection) + Send,
{
    pub(super) fn new(f: F) -> Self {
        Self { f: Some(f) }
    }
}

impl<F> DatabaseThreadWork for AnyDatabaseThreadWork<F>
where
    F: FnOnce(&mut Connection) + Send,
{
    fn perform(&mut self, conn: &mut Connection) {
        let f = self.f.take().unwrap();
        f(conn)
    }
}
