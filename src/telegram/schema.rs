use teloxide::{
    dispatching::{
        dialogue::{self, InMemStorage},
        UpdateFilterExt, UpdateHandler,
    },
    dptree::case,
    types::Update,
};

use super::{states, types};
use types::{Command, State};

use states::{cancel, get_name, night_selection, phone_number, recieve_messages, start, zone};

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![State::Start].branch(case![Command::Start].endpoint(start::main)))
        .branch(case![Command::Cancel].endpoint(cancel::main));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::RecievePhoneNumber { night }].endpoint(phone_number::main))
        .branch(
            case![State::RecieveName {
                night,
                phone_number
            }]
            .endpoint(get_name::main),
        )
        .branch(case![State::RecieveMessages { thread_id }].endpoint(recieve_messages::main));

    let callback_query_handler = Update::filter_callback_query()
        .branch(case![State::Start].endpoint(night_selection::main))
        .branch(
            case![State::RecieveZone {
                night,
                phone_number,
                name,
                username
            }]
            .endpoint(zone::main),
        );
    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}
