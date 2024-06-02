use teloxide::{requests::Requester, types::CallbackQuery, Bot};

use crate::telegram::types::{HandlerResult, MyDialogue, State};

pub async fn main(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
    if let Some(night) = &q.data {
        bot.send_message(dialogue.chat_id(), format!("You have chosen {night}"))
            .await?;
        bot.send_message(dialogue.chat_id(), "Please enter your phone number.")
            .await?;

        dialogue
            .update(State::RecievePhoneNumber {
                night: night.to_owned(),
            })
            .await?;
    }

    Ok(())
}
