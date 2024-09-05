use std::future::Future;

use async_trait::async_trait;
use teloxide::{
    dispatching::DpHandlerDescription,
    dptree::{entry, Handler},
    prelude::DependencyMap,
};

use crate::utils::Result;

type TeloxideHandler = Handler<'static, DependencyMap, Result<()>, DpHandlerDescription>;

pub struct Command {
    pub cmd: String,
    pub desc: String,
    pub handler: TeloxideHandler,
    pub is_hidden: bool,
}

impl Command {
    pub fn new(command: &str, description: &str, handler: TeloxideHandler) -> Self {
        Self {
            cmd: String::from(command),
            desc: String::from(description),
            handler,
            is_hidden: false,
        }
    }

    pub fn hide(mut self) -> Self {
        self.is_hidden = true;
        self
    }
}

#[async_trait]
pub trait Module {
    async fn register(&mut self, deps: &mut DependencyMap) -> Result<()>;

    fn filter(&self) -> TeloxideHandler {
        entry()
    }

    fn commands(&self) -> Vec<Command> {
        vec![]
    }
}

#[derive(Default)]
pub struct ModuleManager {
    mods: Vec<Box<dyn Module>>,
}

impl ModuleManager {
    pub fn new() -> Self {
        Self { mods: Vec::new() }
    }

    pub fn register_modules<M: Module + 'static>(&mut self, module: M) {
        self.mods.push(Box::new(module))
    }

    pub fn with_all_modules<F: FnMut(&mut dyn Module)>(&mut self, mut f: F) {
        let _ = self.mods.iter_mut().map(|m| f(m.as_mut()));
    }

    pub async fn with_all_modules_async<'a, F, Fut>(&'a mut self, mut f: F) -> Result<()>
    where
        F: FnMut(&'a mut dyn Module) -> Fut,
        Fut: Future<Output = Result<()>> + 'a,
    {
        for module in self.mods.iter_mut() {
            f(module.as_mut()).await?
        }
        Ok(())
    }
}
