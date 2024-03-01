#![allow(unused)]

mod commands;
use commands::Command;

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
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(commands_handler),
        )
        .branch(Update::filter_message().endpoint(text_message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    Dispatcher::builder(bot, handler).build().dispatch().await;
}

async fn commands_handler(bot: Bot, msg: Message, cmd: Command) -> HandlerResult {
    log::info!("{:?}", &cmd);

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }

        Command::Dice => bot.send_dice(msg.chat.id).await?,
        Command::Start => todo!(),
    };

    // // Create a simple inline keyboard with a single button
    // let inline_keyboard =
    //     InlineKeyboardMarkup::default().append_row(vec![InlineKeyboardButton::callback(
    //         "Roll Dice",
    //         "/roll_dice",
    //     )]);

    // // Send the message with the inline keyboard
    // bot.send_message(msg.chat.id, "Click the button to roll the dice.")
    //     .reply_markup(inline_keyboard)
    //     .await?;

    Ok(())
}

async fn text_message_handler(bot: Bot, msg: Message) -> HandlerResult {
    println!("text_message_handler");
    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> HandlerResult {
    println!("callback_handler");
    Ok(())
}
