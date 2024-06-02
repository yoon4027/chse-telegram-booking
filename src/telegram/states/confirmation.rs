// use mongodb::Database;
// use teloxide::{requests::Requester, types::CallbackQuery, Bot};

// use crate::telegram::types::{HandlerResult, MyDialogue, Person, State};

// pub async fn main(
//     bot: Bot,
//     dialogue: MyDialogue,
//     (name, night, phone_number, zone): (String, String, String, String),
//     q: CallbackQuery,
//     db: Database,
// ) -> HandlerResult {
//     if let Some(confirm) = &q.data {
//         if confirm == "No" {
//             bot.send_message(dialogue.chat_id(), "Alright, starting all over again.")
//                 .await?;

//             dialogue.update(State::Start).await?;

//             return Ok(());
//         }

//         db.collection("pending_approvals")
//             .insert_one(
//                 Person {
//                     name: name.clone(),
//                     phone_number: phone_number.clone(),
//                     zone: zone.to_string(),
//                     night: night.clone(),
//                     chat_id: dialogue.chat_id().to_string(),
//                 },
//                 None,
//             )
//             .await?;

//         bot.send_message(
//             dialogue.chat_id(),
//             "Okay, cool. You chose not to make any changes",
//         )
//         .await?;

//         bot.send_message(dialogue.chat_id(), "Thank you for reaching out! One of our EXCO members will be in touch with you shortly :)").await?;

//         dialogue.exit().await?;
//     }

//     Ok(())
// }
