use eyre::Result;
use mongodb::Database;
use serenity::{all::GatewayIntents, http::Http, prelude::TypeMapKey, Client};
use std::sync::{Arc, Mutex};
use teloxide::Bot;

use crate::telegram::DiscordMap;

mod events;

#[derive(Clone)]
pub struct DiscordState {
    pub db: Database,
    pub discord_map: DiscordMap,
    pub bot: Option<Bot>,
}

impl TypeMapKey for DiscordState {
    type Value = Arc<Mutex<DiscordState>>;
}

pub async fn start_discord(
    db: Database,
    discord_map: DiscordMap,
) -> Result<(Arc<Http>, Arc<Mutex<DiscordState>>)> {
    println!("Called");

    let mut client = Client::builder("", GatewayIntents::all())
        .event_handler(events::Handler)
        .await?;

    let http = client.http.clone();
    let state = Arc::new(Mutex::new(DiscordState {
        db,
        discord_map,
        bot: None,
    }));

    {
        let mut data = client.data.write().await;

        data.insert::<DiscordState>(state.clone());
    }

    tokio::spawn(async move {
        client.start().await.unwrap();
    });

    Ok((http, state))
}
