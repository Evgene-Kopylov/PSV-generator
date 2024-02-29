#![allow(unused)]

use dotenv_codegen::dotenv;
use patience_lib::patience::MySpread;

use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    log::info!("Starting throw dice bot...");

    let bot = Bot::new(dotenv!("TELOXIDE_TOKEN"));

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
    #[command(description = "1D6")]
    Dice,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}."))
                .await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(
                msg.chat.id,
                format!("Your username is @{username} and age is {age}."),
            )
            .await?
        }
        Command::Dice => bot.send_dice(msg.chat.id).await?,
    };

    Ok(())
}
