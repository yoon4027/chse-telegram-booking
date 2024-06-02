use std::sync::Arc;

use serenity::all::{ChannelId, CreateAttachment, ExecuteWebhook, Http, Webhook};
use std::io::Cursor;
use teloxide::{net::Download, requests::Requester, types::Message, Bot};
use uuid::Uuid;

use crate::telegram::types::HandlerResult;

pub async fn main(
    bot: Bot,
    msg: Message,
    thread_id: ChannelId,
    discord: Arc<Http>,
) -> HandlerResult {
    let webhook = Webhook::from_id_with_token(
        &discord,
        1010,
        "",
    )
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

    let mut d_message = ExecuteWebhook::new()
        .in_thread(thread_id)
        .username(username);

    if let Some(text) = msg.text() {
        d_message = d_message.content(text.to_string());
    }

    if let Some(photos) = msg.photo() {
        for (i, photo) in photos.iter().enumerate() {
            let file = bot.get_file(&photo.file.id).await?;
            let mut buf = Cursor::new(Vec::<u8>::new());
            bot.download_file(&file.path, &mut buf).await?;

            d_message = d_message.add_file(CreateAttachment::bytes(
                buf.get_ref().to_vec(),
                format!("{}-photo{}.jpeg", Uuid::new_v4(), i),
            ));
        }
    }

    if let Some(document) = msg.document() {
        let file_id = document.file.id.clone();
        let file = bot.get_file(file_id).await?;

        let mut buf = Cursor::new(Vec::<u8>::new());

        bot.download_file(&file.path, &mut buf).await?;

        d_message = d_message.add_file(CreateAttachment::bytes(
            buf.get_ref().to_vec(),
            format!(
                "{}-{}",
                Uuid::new_v4(),
                document.file_name.clone().unwrap_or_else(|| "".to_string())
            ),
        ));
    }

    webhook.execute(&discord, false, d_message).await?;

    Ok(())
}
