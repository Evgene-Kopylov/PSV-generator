#![allow(unused)]

use dotenv_codegen::dotenv;
use patience_lib::patience::MySpread;

use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message, ParseMode},
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

                "/dice_btn" => {
                    // Create a simple inline keyboard with a single button
                    let inline_keyboard = InlineKeyboardMarkup::default().append_row(vec![
                        InlineKeyboardButton::callback("Roll Dice", "/roll_dice"),
                    ]);

                    // Send the message with the inline keyboard
                    bot.send_message(msg.chat.id, "Click the button to roll the dice.")
                        .reply_markup(inline_keyboard)
                        .await?
                }

                _ => bot.send_message(msg.chat.id, "/dice /dice_btn").await?,
            };
        };
        Ok(())
    })
    .await;
}
