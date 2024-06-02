use mongodb::error::{ErrorKind, WriteFailure};
use qrcode_generator::QrCodeEcc;
use serenity::{
    all::{Context, EventHandler, Message, Ready},
    async_trait,
};
use teloxide::{
    requests::Requester,
    types::{InputFile, Recipient},
};

use url::Url;

use super::DiscordState;

use crate::{
    model::{Seat, UserTicketOrder},
    telegram::HashStructure,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, __: Ready) {
        println!("Discord is running.");
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let state = ctx
            .data
            .read()
            .await
            .get::<DiscordState>()
            .unwrap()
            .lock()
            .unwrap()
            .clone();

        let hash_structure = match state.discord_map.lock().await.get(&msg.channel_id) {
            Some(n) => n.clone(),
            None => return,
        };

        match parse_command(&msg.content) {
            Ok(Some((night1, night2))) => {
                approve_command(state, ctx, msg, hash_structure, night1, night2).await;
                return;
            }
            Ok(None) => {
                // It is not the .approve command, continue
            }
            Err(e) => {
                msg.reply(&ctx.http, e).await.ok();
                return;
            }
        }

        // If the text is empty and theres no attachments:
        if msg.content.is_empty() && msg.attachments.is_empty() {
            msg.reply(
                &ctx.http,
                "You can only send text and images to users right now!",
            )
            .await
            .ok();

            return;
        }

        // If there's an attachment/image send it.
        if !msg.attachments.is_empty() {
            for image in &msg.attachments {
                if let Err(e) = state
                    .bot
                    .as_ref()
                    .unwrap()
                    .send_photo(
                        Recipient::Id(hash_structure.telegram_id.clone()),
                        InputFile::url(Url::parse(&image.url).unwrap()),
                    )
                    .await
                {
                    msg.reply(
                        &ctx.http,
                        format!("Failed to send message to telegram channel: {}", e),
                    )
                    .await
                    .ok();
                }
            }
        }

        if let Err(e) = state
            .bot
            .as_ref()
            .unwrap()
            .send_message(
                Recipient::Id(hash_structure.telegram_id.clone()),
                &msg.content,
            )
            .await
        {
            msg.reply(
                &ctx.http,
                format!("Failed to send message to telegram channel: {}", e),
            )
            .await
            .ok();
        }
    }
}

async fn approve_command(
    state: DiscordState,
    ctx: Context,
    msg: Message,
    hash_structure: HashStructure,
    night1: Vec<String>,
    night2: Vec<String>,
) {
    let db_result = state
        .db
        .collection::<UserTicketOrder>("approved_orders")
        .insert_one(
            UserTicketOrder {
                user_info: hash_structure.user_info.clone(),
                seats: vec![
                    night1
                        .clone()
                        .into_iter()
                        .map(|x| Seat {
                            seat_number: format!("night1-{}", x),
                        })
                        .collect::<Vec<_>>(),
                    night2
                        .clone()
                        .into_iter()
                        .map(|x| Seat {
                            seat_number: format!("night2-{}", x),
                        })
                        .collect(),
                ]
                .concat(),
            },
            None,
        )
        .await;

    let object_id = match db_result {
        Ok(n) => n.inserted_id.as_object_id().unwrap(),
        Err(e) => {
            match e.kind.as_ref() {
                ErrorKind::Write(n) => match n {
                    WriteFailure::WriteError(n) => {
                        if n.code == 11000 {
                            msg.reply(&ctx.http, "One of those seats is already booked!")
                                .await
                                .ok();

                            return;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }

            msg.reply(&ctx.http, &format!("Error from DB: {}", e))
                .await
                .ok();

            return;
        }
    };

    let verify_url = format!("https://garujam.com/verify/{}", object_id.to_hex());

    let qr_code_image =
        qrcode_generator::to_png_to_vec(verify_url, QrCodeEcc::Medium, 1024).unwrap();

    state
        .bot
        .clone()
        .unwrap()
        .send_message(
            Recipient::Id(hash_structure.telegram_id.clone()),
            format!(
                "Booking seats...\nNight 1 seats: {}\nNight 2 seats: {}",
                night1.join(", "),
                night2.join(", ")
            ),
        )
        .await
        .ok();

    if let Err(e) = state
        .bot
        .clone()
        .unwrap()
        .send_photo(
            Recipient::Id(hash_structure.telegram_id.clone()),
            InputFile::memory(qr_code_image),
        )
        .await
    {
        msg.reply(
            &ctx.http,
            &format!("Error sending QR code image to Telegram: {}", e),
        )
        .await
        .ok();

        return;
    }

    msg.reply(&ctx.http, "Successfully sent QR code to them!")
        .await
        .ok();
}

fn parse_command(msg: &str) -> Result<Option<(Vec<String>, Vec<String>)>, String> {
    // .approve night1 a3,a5,d6,vip-a5 night2 a4,a5
    // .approve night2 a6

    if !msg.starts_with(".approve") {
        return Ok(None);
    }

    let command_body = msg.replacen(".approve", "", 1).trim().to_string();

    let mut night1: Vec<String> = vec![];
    let mut night2: Vec<String> = vec![];

    let mut split = command_body.split(' ');

    loop {
        let night = match split.next() {
            Some(n) => n,
            None => break,
        };

        let nights_array = match night {
            "night1" => &mut night1,
            "night2" => &mut night2,
            _ => return Err(format!("Invalid night `{}`", night)),
        };

        let seats = match split.next() {
            Some(n) => n,
            None => {
                return Err(format!(
                    "Expected comma-separated list of seats after night `{}`",
                    night
                ));
            }
        };

        for seat in seats.split(',') {
            let seat = seat.to_string();

            if !is_valid_seat_number(&seat) {
                return Err(format!("Invalid seat `{}`", seat));
            }

            if nights_array.contains(&seat) {
                return Err(format!("You already mentioned seat `{}`", seat));
            }

            nights_array.push(seat.to_string());
        }
    }

    if night1.len() == 0 && night2.len() == 0 {
        return Err("You must specify some seats".to_string());
    }

    Ok(Some((night1, night2)))
}

pub fn is_valid_seat_number(seat_number: &str) -> bool {
    let regex_expression = match seat_number.starts_with("VIP-") {
        true => {
            let suffix = seat_number.strip_prefix("VIP-").unwrap();

            match suffix.starts_with("A") || suffix.starts_with("B") {
                true => "^VIP-[A-B]([1-6]|1[1-6])$",
                false => "^VIP-[C-D]([1-9]|1[0-6])$",
            }
        }
        false => "^[A-Z]([1-9]|1[0-9]|2[0-4])$",
    };

    regex::Regex::new(regex_expression)
        .unwrap()
        .is_match(seat_number)
}
