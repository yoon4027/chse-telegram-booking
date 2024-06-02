use std::sync::Arc;

use mongodb::Database;
use serenity::all::Http;
use teloxide::{
    dispatching::{dialogue::InMemStorage, Dispatcher},
    dptree, Bot,
};

pub use self::types::{DiscordMap, HashStructure, State};

mod schema;
mod states;
mod types;

pub async fn start_telegram(db: Database, discord: Arc<Http>, discord_map: DiscordMap) -> Bot {
    let bot = Bot::new("");

    let bot0 = bot.clone();

    tokio::spawn(async move {
        Dispatcher::builder(bot0, schema::schema())
            .dependencies(dptree::deps![
                InMemStorage::<State>::new(),
                db,
                discord,
                discord_map
            ])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    });

    bot
}
