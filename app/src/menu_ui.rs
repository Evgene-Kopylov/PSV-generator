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
    let mut text = String::new();
    if tg_contact.active_index.is_some() {
        text += &format!(
            "Карта № {} из {}",
            tg_contact.clone().active_index.unwrap() + 1,
            chain.len()
        );
    } else {
        text += &format!("Цепочка {} карт", chain.len());
    }
    let row = vec![InlineKeyboardButton::callback(text, "info_chain")];
    keyboard.push(row);
    // грид цепочки
    let mut index = 0;
    for chank in chain.chunks(btn_row_size) {
        let mut row = vec![];
        for item in chank {
            let mut card_text = String::new();
            let callback_data = format!("item_{}", &index);

            if let Some(card) = item {
                card_text += &format!(
                    "{}{}",
                    card.clone().rank.unwrap_or("_".to_string()),
                    card.clone().suit.unwrap_or("_".to_string()),
                );
            } else {
                card_text += "  ";
            }

            if let Some(active_index) = tg_contact.active_index {
                if active_index == index {
                    if card_text.starts_with("  ") {
                        card_text = format!("_ _");
                    } else {
                        card_text = format!(">{}", card_text);
                    }
                }
            }
            row.push(InlineKeyboardButton::callback(card_text, callback_data));
            index += 1;
        }

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
