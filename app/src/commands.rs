use crate::error_handler::{HandlerMessage, HandlerResult};
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

async fn start(bot: Bot, msg: Message) -> HandlerMessage {
    // // Create a simple inline keyboard with a single button
    // let mut inline_keyboard = InlineKeyboardMarkup::default();

    // for i in 0..15 {
    //     inline_keyboard
    //         .clone()
    //         .append_row(vec![InlineKeyboardButton::callback(
    //             "Roll Dice",
    //             "/roll_dice",
    //         )]);
    // }

    // // Send the message with the inline keyboard
    // Ok(bot
    //     .send_message(msg.chat.id, "Click the button to roll the dice.")
    //     .reply_markup(inline_keyboard)
    //     .await?)

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
    let chank_size = 5;
    let mut ranks = ranks.clone();
    let reminder = chank_size - ranks.len() % chank_size;
    dbg!(reminder);
    if ranks.len() % chank_size > 0 && reminder > 0 {
        ranks.extend(std::iter::repeat(" ").take(reminder));
    }

    // Информационная кнопка
    let row = vec![InlineKeyboardButton::callback("Ранги", "ranks")];
    keyboard.push(row);

    // грид кнопок рангов
    for rank in ranks.chunks(chank_size) {
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
    if suits.len() < chank_size {
        suits.extend(std::iter::repeat(" ").take(chank_size - suits.len()));
    }

    // for suit in suits {
    let row = suits
        .iter()
        .map(|&item| InlineKeyboardButton::callback(item, "suit_".to_owned() + item))
        .collect();

    keyboard.push(row);
    // }

    InlineKeyboardMarkup::new(keyboard)
}
