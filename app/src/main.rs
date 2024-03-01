#![allow(unused)]

use std::error::Error;

use dotenv_codegen::dotenv;
use patience_lib::patience::MySpread;

use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Update},
    utils::command::BotCommands,
};

type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    log::info!("Начало работы...");

    let bot = Bot::new(dotenv!("TELOXIDE_TOKEN"));

    let handler = dptree::entry()
        .branch(Update::filter_message().filter_command::<Cmd>().endpoint(b))
        .branch(Update::filter_message().endpoint(c))
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    Dispatcher::builder(bot, handler).build().dispatch().await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Список команд:")]
enum Cmd {
    B,
}

async fn b(bot: Bot, msg: Message) -> HandlerResult {
    println!("b");

    // Create a simple inline keyboard with a single button
    let inline_keyboard =
        InlineKeyboardMarkup::default().append_row(vec![InlineKeyboardButton::callback(
            "Roll Dice",
            "/roll_dice",
        )]);

    // Send the message with the inline keyboard
    bot.send_message(msg.chat.id, "Click the button to roll the dice.")
        .reply_markup(inline_keyboard)
        .await?;

    Ok(())
}

async fn c() -> HandlerResult {
    println!("c");
    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> HandlerResult {
    println!("callback_handler");
    Ok(())
}
