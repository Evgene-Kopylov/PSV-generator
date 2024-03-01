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
        // .branch(Update::filter_my_chat_member().endpoint(a))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_message().filter_command::<Cmd>().endpoint(b))
        .branch(Update::filter_message().endpoint(c));

    Dispatcher::builder(bot, handler)
        .build()
        // .setup_ctrlc_handler()
        .dispatch()
        .await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Список команд:")]
enum Cmd {
    B,
}

async fn a() -> Result<(), ()> {
    println!("a");
    Ok(())
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

// use teloxide::{
//     dispatching::dialogue::InMemStorage,
//     prelude::*,
//     types::{InlineKeyboardButton, InlineKeyboardMarkup, Message, ParseMode},
//     utils::command::BotCommands,
// };

// #[tokio::main]
// async fn main() {

//     let bot = Bot::new(dotenv!("TELOXIDE_TOKEN"));

//     let handler = Update::filter_message()
//             .enter_dialogue::<Message, InMemStorage<State>, State>()
//             .branch(dptree::case![State::Start].endpoint(start))
//             .branch(dptree::case![State::ReceiveFullName].endpoint(receive_full_name))
//             .branch(dptree::case![State::ReceiveAge { full_name }].endpoint(receive_age))
//             .branch(
//                 dptree::case![State::ReceiveLocation { full_name, age }].endpoint(receive_location),
//             );

//     Dispatcher::builder(
//         bot,
//         handler
//     )
//     .dependencies(dptree::deps![InMemStorage::<State>::new()])
//     .enable_ctrlc_handler()
//     .build()
//     .dispatch()
//     .await;

// let handler2 = {
//     |bot: Bot, msg: Message| async move {
//         if let Some(command) = msg.text() {
//             match command {
//                 "/dice" => bot.send_dice(msg.chat.id).await?,

//                 "/dice_btn" => {
//                     // Create a simple inline keyboard with a single button
//                     let inline_keyboard = InlineKeyboardMarkup::default().append_row(vec![
//                         InlineKeyboardButton::callback("Roll Dice", "/roll_dice"),
//                     ]);

//                     // Send the message with the inline keyboard
//                     bot.send_message(msg.chat.id, "Click the button to roll the dice.")
//                         .reply_markup(inline_keyboard)
//                         .await?
//                 }

//                 _ => bot.send_message(msg.chat.id, "/dice /dice_btn").await?,
//             };
//         };
//         Ok(())
//     }
// };

// teloxide::repl(bot, handler2).await;
// }
