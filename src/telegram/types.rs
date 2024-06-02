use std::{collections::HashMap, sync::Arc};

use serenity::all::ChannelId;
use teloxide::{
    dispatching::dialogue::{Dialogue, InMemStorage},
    macros::BotCommands,
    types::ChatId,
};
use tokio::sync::Mutex;

use crate::model::UserInfo;

pub type MyDialogue = Dialogue<State, InMemStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
pub type DiscordMap = Arc<Mutex<HashMap<ChannelId, HashStructure>>>;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Start,
    Cancel,
}

#[derive(Clone, Default, Debug)]
pub enum State {
    #[default]
    Start,
    RecievePhoneNumber {
        night: String,
    },
    RecieveName {
        night: String,
        phone_number: String,
    },
    RecieveZone {
        night: String,
        phone_number: String,
        name: String,
        username: String,
    },
    RecieveMessages {
        thread_id: ChannelId,
    },
}

#[derive(Debug, Clone)]
pub struct HashStructure {
    pub telegram_id: ChatId,
    pub user_info: UserInfo,
}
