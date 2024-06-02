use discord::start_discord;
use eyre::Result;
use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Database,
};
use serenity::all::ChannelId;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

mod discord;
mod lib;
mod model;
mod telegram;
mod verifier;

use self::telegram::{DiscordMap, HashStructure};

#[tokio::main]
async fn main() -> Result<()> {
    let db = mongodb().await?;
    let discord_map: DiscordMap = Arc::new(Mutex::new(HashMap::<ChannelId, HashStructure>::new()));

    let (discord, discord_state) = start_discord(db.clone(), discord_map.clone()).await?;

    let bot = telegram::start_telegram(db.clone(), discord.clone(), discord_map).await;

    discord_state.lock().unwrap().bot.replace(bot);

    self::verifier::start_verifier_web_server(db).await;

    println!("Finished startup");

    Ok(())
}

async fn mongodb() -> mongodb::error::Result<Database> {
    let mut client_options = ClientOptions::parse("").await?;

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;

    Ok(client.database("bot"))
}
