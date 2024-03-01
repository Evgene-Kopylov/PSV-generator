use crate::errors::{HandlerMessageResult, HandlerResult};
use teloxide::payloads::SendMessageSetters as _;
use teloxide::prelude::Requester;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
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

    Ok(())
}

async fn start(bot: Bot, msg: Message) -> HandlerMessageResult {
    let suits = vec!["☐", "L", "▲", "♡", "○"];
    let ranks = vec![
        "T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛",
    ];

    let keyboard = make_keyboard(suits, ranks);
    Ok(bot
        .send_message(msg.chat.id, "Пасьянс Симпатии и Валентности.")
        .reply_markup(keyboard)
        .await?)
}

fn make_keyboard(suits: Vec<&str>, ranks: Vec<&str>) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    // дополнить список рангов до кратной числу кнопок в ряду длянны.
    let btn_row_size = 5;
    let mut ranks = ranks.clone();
    let reminder = btn_row_size - ranks.len() % btn_row_size;
    dbg!(reminder);
    if ranks.len() % btn_row_size > 0 && reminder > 0 {
        ranks.extend(std::iter::repeat(" ").take(reminder));
    }

    // Информационная кнопка
    let row = vec![InlineKeyboardButton::callback("Ранги", "ranks")];
    keyboard.push(row);

    // грид кнопок рангов
    for rank in ranks.chunks(btn_row_size) {
        let row = rank
            .iter()
            .map(|&item| InlineKeyboardButton::callback(item, "rank_".to_owned() + item))
            .collect();

        keyboard.push(row);
    }
    let row = vec![InlineKeyboardButton::callback("Масти", "suits")];
    keyboard.push(row);

    // Дополнить список мастей до кратной числу кнопок в ряду длинны.
    let mut suits = suits.clone();
    if suits.len() < btn_row_size {
        suits.extend(std::iter::repeat(" ").take(btn_row_size - suits.len()));
    }

    // линия мастей.
    let row = suits
        .iter()
        .map(|&item| InlineKeyboardButton::callback(item, "suit_".to_owned() + item))
        .collect();

    keyboard.push(row);

    InlineKeyboardMarkup::new(keyboard)
}
