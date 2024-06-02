use teloxide::{requests::Requester, types::Message, Bot};

use crate::{
    lib::utils::is_valid_number,
    telegram::types::{HandlerResult, MyDialogue, State},
};

pub async fn main(bot: Bot, dialogue: MyDialogue, msg: Message, night: String) -> HandlerResult {
    println!("{night}");

    let message_text = match msg.text() {
        Some(n) => n,
        None => {
            bot.send_message(
                msg.chat.id,
                "It seems like I failed to parse the phone number. Try again.",
            )
            .await?;

            return Ok(());
        }
    };

    if !is_valid_number(message_text) {
        bot.send_message(msg.chat.id, "The phone number is not valid. Try again.")
            .await?;

        return Ok(());
    }

    println!("{message_text}");

    bot.send_message(
        msg.chat.id,
        "Please enter your name. This will be used to identify your booking.",
    )
    .await?;

    dialogue
        .update(State::RecieveName {
            night,
            phone_number: message_text.to_string(),
        })
        .await?;

    Ok(())
}
