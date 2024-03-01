#![allow(unused)]

mod commands;
mod errors;
use commands::{commands_handler, Command};
use errors::HandlerResult;

use dotenv_codegen::dotenv;
use patience_lib::patience::MySpread;

use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Update},
    utils::command::BotCommands,
};

#[tokio::main]
async fn main() {
    log::info!("Начало работы...");

    let bot = Bot::new(dotenv!("TELOXIDE_TOKEN"));

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(commands_handler),
        )
        .branch(Update::filter_message().endpoint(text_message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    Dispatcher::builder(bot, handler).build().dispatch().await;
}

async fn text_message_handler(bot: Bot, msg: Message) -> HandlerResult {
    println!("text_message_handler");
    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> HandlerResult {
    println!("callback_handler");
    println!("q.data = {:?}", q.data.unwrap_or("Пусто".to_string()));
    Ok(())
}
