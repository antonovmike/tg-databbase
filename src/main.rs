// #![allow(unused)]
// use std::ops::Not;
// use carapax::types::User;
use carapax::types::{
    Message, Text, 
    // KeyboardButton, InlineKeyboardButton, InputFile, MessageData, TextEntity
};
use carapax::{
    longpoll::LongPoll,
    methods::SendMessage,
    types::{
        ChatId, 
        // Text, Update
    },
    Api, App, Context, ExecuteError, Ref,
};
use dotenv::dotenv;
// use serde::ser::Error;
// use serde::{Deserialize, Serialize};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let api = Api::new(token).expect("Failed to create API");

    let mut context = Context::default();
    context.insert(api.clone());

    let app = App::new(context, echo);
    LongPoll::new(api, app).run().await
}

async fn echo(api: Ref<Api>, chat_id: ChatId, message: Message) -> Result<(), ExecuteError> {
    let content = message.get_text().unwrap();
    let string_slice = &content.data[..=5];
    match string_slice {
        "/start" => api.execute(SendMessage::new(chat_id.clone(), "Received /start")).await?,
        "/dbadd" => api.execute(SendMessage::new(chat_id.clone(), "Received /dbadd")).await?,
        _ => api.execute(SendMessage::new(chat_id.clone(), "Error")).await?,
    };
    Ok(())
}
