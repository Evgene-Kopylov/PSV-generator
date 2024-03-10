use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

use crate::TexoxideError;
use crate::TgContact;

pub const BTN_PLACEHOLDER: &str = "∘";

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

    // Дополнить список мастей до кратной числу кнопок в ряду длинны.
    let mut suits = tg_contact.clone().suits;
    if suits.len() < btn_row_size {
        suits.extend(std::iter::repeat(" ".to_string()).take(btn_row_size - suits.len()));
    }

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

    keyboard.push(row);

    let chain = tg_contact.clone().chain;

    // Цепочка
    let mut text = String::new();
    if tg_contact.chain_index.is_some() {
        text += &format!(
            "Карта № {} из {}",
            tg_contact.clone().chain_index.unwrap() + 1,
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
            let callback_data = format!("chain_{}", &index);

            if let Some(card) = item {
                card_text += &format!(
                    "{}{}",
                    card.clone().rank.unwrap_or("_".to_string()),
                    card.clone().suit.unwrap_or("_".to_string()),
                );
            } else {
                card_text += "  ";
            }

            if let Some(active_index) = tg_contact.chain_index {
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

        // дополнить линию заглушками
        for _ in 1..btn_row_size {
            if row.len() < btn_row_size {
                // ☠   ⌧  ⌲  ⍁   ╳  ＞＜   ＞∘＜   ＞○＜"
                let text = BTN_PLACEHOLDER;
                let callback_data = "empty";
                row.push(InlineKeyboardButton::callback(text, callback_data))
            }
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
