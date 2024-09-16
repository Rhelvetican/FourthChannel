use teloxide::{
    dispatching::DpHandlerDescription,
    dptree::{entry, Handler},
    prelude::DependencyMap,
};

use crate::util::Result;

pub type FcHandler<'a, T, Desc> = Handler<'a, DependencyMap, Result<T>, Desc>;

pub fn handler() -> FcHandler<'static, (), DpHandlerDescription> {
    entry()
}
