use post::post;
use teloxide::{
    dispatching::{UpdateFilterExt, UpdateHandler},
    prelude::dptree::entry,
    types::Update,
};

mod post;

use crate::utils::Error;

pub fn handlers() -> UpdateHandler<Error> {
    entry().branch(Update::filter_message().endpoint(post))
}
