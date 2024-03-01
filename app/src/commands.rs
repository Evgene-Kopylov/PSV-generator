use crate::error_handler::{HandlerMessage, HandlerResult};
use teloxide::prelude::Requester;
use teloxide::{types::Message, utils::command::BotCommands, Bot};

#[derive(BotCommands, Debug, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "Список поддерживаемых команд:"
)]
pub enum Command {
    #[command(description = "Этот текст.")]
    Help,
    #[command(description = "Кубик Д6")]
    Dice,
    #[command(description = "Запустить бот.")]
    Start,
}

pub async fn commands_handler(bot: Bot, msg: Message, cmd: Command) -> HandlerResult {
    log::info!("{:?}", &cmd);

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }

        Command::Dice => bot.send_dice(msg.chat.id).await?,
        Command::Start => start(bot, msg).await?,
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

async fn start(bot: Bot, msg: Message) -> HandlerMessage {
    println!("start");
    Ok(bot.send_message(msg.chat.id, "Start").await?)
}
