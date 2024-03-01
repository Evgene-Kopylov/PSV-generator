#![allow(unused)]

mod commands;
mod custom_error_handler;
use commands::{commands_handler, Command};
use custom_error_handler::HandlerResult;

use dotenv_codegen::dotenv;
use patience_lib::patience::MySpread;

use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Update},
    utils::command::BotCommands,
};

// type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

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
    Ok(())
}
