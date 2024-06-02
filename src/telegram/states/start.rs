use teloxide::{
    payloads::SendMessageSetters,
    requests::Requester,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message},
    Bot,
};

use crate::telegram::types::{HandlerResult, MyDialogue};

pub async fn main(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let days = ["Night 1", "Night 2", "Both"]
        .map(|product| InlineKeyboardButton::callback(product, product));

    bot.send_message(msg.chat.id, "Hello! Which night would you like to choose?")
        .await?;

    bot.send_message(
        msg.chat.id,
        "Night 1 - Dhiriulhumakee miee baa - March 7\n\nNight 2 - Folk Tales - March 8\n\nBoth",
    )
    .reply_markup(InlineKeyboardMarkup::new([days]))
    .await?;

    Ok(())
}
