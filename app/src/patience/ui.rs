use teloxide::{prelude::Bot, requests::Requester};

use crate::{State, TeloxideDialogue, TexoxideError, TgContact};
use teloxide::types::ParseMode;
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

pub async fn spawn_patience_chain(
    bot: Bot,
    // msg: Message,
    dialoque: TeloxideDialogue,
    mut tg_contact: TgContact,
) -> Result<Message, TexoxideError> {
    let mut patience = tg_contact.clone().patience.unwrap();

    let text = format!(
        "<span class='tg-spoiler'>{}</span>\n### Сведение",
        patience.iteration
    );

    let keyboard = make_keyboard(tg_contact.clone());
    let message: Message = bot
        .parse_mode(ParseMode::Html)
        .send_message(tg_contact.clone().menu_msg.unwrap().chat.id, text)
        .reply_markup(keyboard)
        .await?;
    patience.patience_msg = Some(message.clone());
    tg_contact.patience = Some(patience);
    dialoque
        .update(State::Patience {
            tg_contact: tg_contact.clone(),
        })
        .await?;
    Ok(message)
}

pub fn make_keyboard(tg_contact: TgContact) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    // дополнить список рангов до кратной числу кнопок в ряду длянны.
    let btn_row_size = 5;
    let patience = tg_contact.clone().patience.unwrap();
    let chain = patience.chain;

    // грид кнопок рангов
    let mut i = 0;
    for _ in chain.chunks(btn_row_size) {
        let mut row = vec![];
        for _ in 0..btn_row_size {
            if i == chain.len() {
                row.push(InlineKeyboardButton::callback(" ", "empty"));
                continue;
            }
            let card = chain[i].clone();
            let text = format!("{}{}", card.rank.unwrap(), card.suit.unwrap());
            let callback_data = format!("card_{}", i);
            row.push(InlineKeyboardButton::callback(text, callback_data));
            i += 1;
        }

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub async fn update_patience(
    bot: Bot,
    dialogue: TeloxideDialogue,
    mut tg_contact: TgContact,
) -> Result<(), TexoxideError> {
    log::trace!("Обновляю расклад");
    // сбросить активный индекс
    tg_contact.patience_index = None;

    // собрать новуыю клавиатуру
    let keyboard = make_keyboard(tg_contact.clone());

    let mut patience = tg_contact.patience.clone().unwrap();
    let msg = patience.patience_msg.unwrap();
    let old_keyboard = msg.reply_markup().unwrap();
    let new_keyboard = &keyboard;

    // если изменения в клавиатуре, применить
    if old_keyboard != new_keyboard {
        log::trace!("New keyboard");
        let msg_id = msg.id;
        let message = bot
            .edit_message_reply_markup(dialogue.chat_id(), msg_id)
            .reply_markup(keyboard)
            .await?;
        patience.patience_msg = Some(message);
        tg_contact.patience = Some(patience);
        dialogue.update(State::Patience { tg_contact }).await?;
    }
    Ok(())
}
