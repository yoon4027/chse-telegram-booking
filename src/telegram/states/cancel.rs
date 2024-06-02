use crate::telegram::types::{HandlerResult, MyDialogue};
use teloxide::{requests::Requester, types::Message, Bot};

pub async fn main(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Cancelling the dialogue.")
        .await?;
    dialogue.exit().await?;
    Ok(())
}
