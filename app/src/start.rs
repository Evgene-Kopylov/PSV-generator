use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

use crate::TeloxideDialogue;
use crate::TexoxideError;
use crate::{State, TgContact};

pub async fn start(
    bot: Bot,
    dialoque: TeloxideDialogue,
    msg: Message,
) -> Result<(), TexoxideError> {
    log::trace!("Start");
    let tg_contact = TgContact::new();
    spawn_menu(bot, msg, tg_contact.clone()).await?;
    dialoque.update(State::Menu { tg_contact }).await?;
    log::trace!("Произошел спавн меню.");
    Ok(())
}

/// # спавн меню
pub async fn spawn_menu(
    bot: Bot,
    msg: Message,
    tg_contact: TgContact,
) -> Result<(), TexoxideError> {
    let text = "Пасьянс Симпатии и Валентности.";

    let keyboard = make_keyboard(tg_contact);
    let _message: Message = bot
        .send_message(msg.chat.id, text)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

/// # Разметка клавиш
pub fn make_keyboard(tg_contact: TgContact) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    // дополнить список рангов до кратной числу кнопок в ряду длянны.
    let btn_row_size = 5;
    let mut ranks = tg_contact.clone().ranks;
    let reminder = btn_row_size - ranks.len() % btn_row_size;
    if ranks.len() % btn_row_size > 0 && reminder > 0 {
        ranks.extend(std::iter::repeat(" ".to_string()).take(reminder));
    }

    // Информационная кнопка
    let row = vec![InlineKeyboardButton::callback("Ранги", "info_ranks")];
    keyboard.push(row);

    // грид кнопок рангов
    for rank in ranks.chunks(btn_row_size) {
        let row = rank
            .iter()
            .map(|item| InlineKeyboardButton::callback(item, "rank_".to_owned() + &item))
            .collect();

        keyboard.push(row);
    }
    let row = vec![InlineKeyboardButton::callback("Масти", "info_suits")];
    keyboard.push(row);

    // Дополнить список мастей до кратной числу кнопок в ряду длинны.
    let mut suits = tg_contact.clone().suits;
    if suits.len() < btn_row_size {
        suits.extend(std::iter::repeat(" ".to_string()).take(btn_row_size - suits.len()));
    }

    // линия мастей.
    let row = suits
        .iter()
        .map(|item| InlineKeyboardButton::callback(item, "suit_".to_owned() + &item))
        .collect();
    keyboard.push(row);

    let chain = tg_contact.clone().chain;

    // Цепочка
    let row = vec![InlineKeyboardButton::callback("Цепочка", "info_chain")];
    keyboard.push(row);
    // грид цепочки
    for row_of_cards in chain.chunks(btn_row_size) {
        let row = row_of_cards
            .iter()
            .map(|_item| InlineKeyboardButton::callback("item", "card_".to_owned()))
            .collect();

        keyboard.push(row);
    }

    // Задать целевую последовательность и сложить пасьянс.
    let row = vec![
        InlineKeyboardButton::callback("➖", "-"),
        InlineKeyboardButton::callback("➕", "+1"),
        InlineKeyboardButton::callback("➕➕", "+5"),
        InlineKeyboardButton::callback(">>>", ">>>"),
    ];
    keyboard.push(row);

    InlineKeyboardMarkup::new(keyboard)
}
