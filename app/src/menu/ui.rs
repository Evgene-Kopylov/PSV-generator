use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

use crate::TgContact;
use crate::{structs::tg_contact::Select, TexoxideError};

/// # спавн меню
pub async fn spawn_menu(
    bot: Bot,
    msg: Message,
    tg_contact: TgContact,
) -> Result<Message, TexoxideError> {
    let text = "Пасьянс Симпатии и Валентности.";

    let keyboard = make_keyboard(tg_contact);
    let message: Message = bot
        // .parse_mode(ParseMode::Html)
        .send_message(msg.chat.id, text)
        .reply_markup(keyboard)
        .await?;
    Ok(message)
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

    // линия мастей.
    let mut row = vec![];

    for i in 0..tg_contact.suits.len() {
        let suit = &tg_contact.clone().suits[i];
        let mut text = suit.clone();
        if let Some(index) = tg_contact.suit_index {
            if i == index {
                text = format!("|{}|", text);
            }
        }
        row.push(InlineKeyboardButton::callback(
            text,
            "suit_".to_owned() + &suit,
        ));
    }

    // дополнить линию мастей
    for _ in 1..btn_row_size {
        if row.len() < btn_row_size {
            let text = " ";
            let callback_data = format!("suit_{}", row.len());
            row.push(InlineKeyboardButton::callback(text, callback_data))
        }
    }

    keyboard.push(row);

    let chain = tg_contact.clone().chain;

    // Цепочка
    let mut text = String::new();
    match tg_contact.select {
        Select::Card { index } => {
            text += &format!(
                "Карта № {} из {}",
                match tg_contact.clone().select {
                    Select::Card { index } => index + 1,
                    _ => 0, // Обработка случая Select::None (или других вариантов)
                },
                chain.len()
            );
        }
        _ => {
            text += &format!("Цепочка {} карт", chain.len());
        }
    }
    let row = vec![InlineKeyboardButton::callback(text, "info_chain")];
    keyboard.push(row);
    // грид цепочки
    let mut i = 0;
    for chank in chain.chunks(btn_row_size) {
        let mut row = vec![];
        for item in chank {
            let mut card_text = String::new();
            let callback_data = format!("chain_{}", &i);

            if let Some(card) = item {
                card_text += &format!(
                    "{}{}",
                    card.clone().rank.unwrap_or("_".to_string()),
                    card.clone().suit.unwrap_or("_".to_string()),
                );
            } else {
                card_text += "  ";
            }

            match tg_contact.select {
                Select::Card { index } => {
                    if index == i {
                        if card_text.starts_with("  ") {
                            card_text = format!("|_ _|");
                        } else {
                            card_text = format!("|{}|", card_text);
                        }
                    }
                }
                Select::Suit { index } => {}
                Select::None => {}
            }
            row.push(InlineKeyboardButton::callback(card_text, callback_data));
            i += 1;
        }

        // дополнить линию заглушками
        for _ in 1..btn_row_size {
            if row.len() < btn_row_size {
                let text = "∘";
                let callback_data = "empty";
                row.push(InlineKeyboardButton::callback(text, callback_data))
            }
        }

        keyboard.push(row);
    }

    // Задать целевую последовательность и сложить пасьянс.
    let row = vec![
        InlineKeyboardButton::callback("🪣", "clean_suit"),
        InlineKeyboardButton::callback("➖", "-"),
        InlineKeyboardButton::callback("➕", "+1"),
        InlineKeyboardButton::callback("➕➕", "+5"),
    ];
    keyboard.push(row);

    let row = vec![
        InlineKeyboardButton::callback("⟲", "restart"),
        InlineKeyboardButton::callback(">>>", ">>>"),
    ];
    keyboard.push(row);

    InlineKeyboardMarkup::new(keyboard)
}
