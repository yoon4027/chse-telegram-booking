use std::sync::Arc;

use serenity::all::{
    ChannelId, Colour, CreateEmbed, CreateEmbedFooter, CreateMessage, CreateThread, Http,
};
use teloxide::{requests::Requester, types::CallbackQuery, Bot};

use crate::{
    model::UserInfo,
    telegram::types::{DiscordMap, HandlerResult, HashStructure, MyDialogue, State},
};

pub async fn main(
    bot: Bot,
    dialogue: MyDialogue,
    (night, phone_number, name, username): (String, String, String, String),
    q: CallbackQuery,
    discord: Arc<Http>,
    discord_map: DiscordMap,
) -> HandlerResult {
    if let Some(zone) = &q.data {
        bot.send_message(dialogue.chat_id(), format!("You have chosen {zone}"))
            .await?;

        let default_message = format!("Okay, we have recieved your information.\n\nName: {name}\nPhone no: {phone_number}\nNight: {night}\nZone: {zone}");

        let embed = CreateEmbed::new()
            .title("New Request")
            .color(Colour::from(424549))
            .field("Name:", &name, false)
            .field("Phone no:", &phone_number, false)
            .field("Night:", &night, false)
            .field("Zone:", &zone.to_string(), false)
            .footer(CreateEmbedFooter::new(
                "Note that you (EXCO team) can only send TEXT messages to the Telegram channel",
            ));

        bot.send_message(dialogue.chat_id(), default_message)
            .await?;
        bot.send_message(dialogue.chat_id(), "Thank you for reaching out! One of our EXCO members will be in touch with you shortly :)").await?;

        let channel = ChannelId::from(1213766352330817556);

        let thread = channel
            .create_thread(&discord, CreateThread::new(format!("chat-{username}")))
            .await?;

        thread
            .send_message(
                &discord,
                CreateMessage::new()
                    .content(format!("<@&1213766120855838740>"))
                    .add_embed(embed),
            )
            .await?;

        discord_map.lock().await.insert(
            thread.id,
            HashStructure {
                telegram_id: dialogue.chat_id(),
                user_info: UserInfo {
                    name,
                    phone_number,
                    zone: zone.to_string(),
                    night,
                },
            },
        );

        dialogue
            .update(State::RecieveMessages {
                thread_id: thread.id,
            })
            .await?;

        println!("{}", thread.id);
    }

    Ok(())
}
