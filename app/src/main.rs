use teloxide::{
    dispatching::{dialogue::InMemStorage, UpdateFilterExt},
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Update},
};

use dotenv_codegen::dotenv;
use std::error::Error;

type TexoxideError = Box<dyn Error + Send + Sync>;
type TeloxideDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    Menu,
}

#[tokio::main]
async fn main() {
    log::info!("Начало работы...");

    let bot = Bot::new(dotenv!("TELOXIDE_TOKEN"));

    let handler = dptree::entry()
        .enter_dialogue::<Update, InMemStorage<State>, State>()
        .branch(Update::filter_message().branch(dptree::case![State::Start].endpoint(start)));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .build()
        .dispatch()
        .await;
}

async fn start(bot: Bot, dialoque: TeloxideDialogue, msg: Message) -> Result<(), TexoxideError> {
    log::info!("Start");
    spawn_menu(bot, msg).await?;
    dialoque.update(State::Menu).await?;
    log::info!("Произошел спавн меню.");
    Ok(())
}

async fn spawn_menu(bot: Bot, msg: Message) -> Result<(), TexoxideError> {
    let suits = vec!["☐", "L", "▲", "♡", "○"];
    let ranks = vec![
        "T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛",
    ];

    let text = "Пасьянс Симпатии и Валентности.";

    let keyboard = make_keyboard(suits, ranks);
    let _message = bot
        .send_message(msg.chat.id, text)
        .reply_markup(keyboard)
        .await?;
    Ok(())
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

    // Задать целевую последовательность и сложить пасьянс.
    let row = vec![
        InlineKeyboardButton::callback("...", "set_target_chain"),
        InlineKeyboardButton::callback("< __ >", "space_card"),
        InlineKeyboardButton::callback(">>>", "patience"),
    ];
    keyboard.push(row);

    InlineKeyboardMarkup::new(keyboard)
}
