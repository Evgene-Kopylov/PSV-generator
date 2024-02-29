// #![allow(unused)]

use dotenv_codegen::dotenv;
use teloxide::{requests::Requester, types::Message, Bot};

#[tokio::main]
async fn main() {
    log::info!("Starting throw dice bot...");

    let bot = Bot::new(dotenv!("TELOXIDE_TOKEN"));

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}