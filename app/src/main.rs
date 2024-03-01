#![allow(unused)]

use dotenv_codegen::dotenv;
use patience_lib::patience::MySpread;

use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::*,
    types::{Message, ParseMode},
    utils::command::BotCommands,
};

#[tokio::main]
async fn main() {
    log::info!("Начало работы...");

    let bot = Bot::new(dotenv!("TELOXIDE_TOKEN"));

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(command) = msg.text() {
            match command {
                "/dice" => bot.send_dice(msg.chat.id).await?,
                _ => bot.send_message(msg.chat.id, "not /dice").await?,
            };
        }
        Ok(())
    })
    .await;
}
