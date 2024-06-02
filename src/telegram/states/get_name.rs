use teloxide::{
    payloads::SendMessageSetters,
    requests::Requester,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile, Message},
    Bot,
};

use url::Url;

use crate::telegram::types::{HandlerResult, MyDialogue, State};

pub async fn main(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    (night, phone_number): (String, String),
) -> HandlerResult {
    let name = match msg.text() {
        Some(n) => n,
        None => {
            bot.send_message(msg.chat.id, "Seems like you did not enter your name")
                .await?;

            return Ok(());
        }
    };

    println!("{name}");

    bot.send_photo(
        msg.chat.id,
        InputFile::url(Url::parse("https://i.imgur.com/4M6NARS.jpeg")?).file_name("seats.png"),
    )
    .await?;

    let buttons = make_keyboard();

    bot.send_message(msg.chat.id, "What zone would you like to choose?")
        .reply_markup(buttons)
        .await?;

    let username = format!(
        "{}{}{}{}",
        msg.chat.username().unwrap_or_else(|| ""),
        msg.chat.first_name().unwrap_or_else(|| ""),
        msg.chat.last_name().unwrap_or_else(|| ""),
        msg.chat.id.0
    )
    .chars()
    .take(12)
    .collect::<String>();

    dialogue
        .update(State::RecieveZone {
            username,
            name: name.to_string(),
            night: night.clone(),
            phone_number: phone_number.clone(),
        })
        .await?;

    Ok(())
}

fn make_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let zones = ["VIP", "Zone Blue", "Zone Yellow", "Zone Red"];

    for versions in zones.chunks(1) {
        let row = versions
            .iter()
            .map(|&version| InlineKeyboardButton::callback(version.to_owned(), version.to_owned()))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}
