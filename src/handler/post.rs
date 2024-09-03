use crate::utils::Result;

use teloxide::{prelude::Requester, types::Message, Bot};

pub async fn post(bot: Bot, msg: Message) -> Result<()> {
    if let Some(media) = msg.document() {
        let file = bot.get_file(&media.file.id).await?;
    }
    Ok(())
}
