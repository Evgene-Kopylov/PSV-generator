use teloxide::{prelude::Bot, requests::Requester};

use crate::{State, TeloxideDialogue, TexoxideError, TgContact};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

fn make_text(tg_contact: TgContact) -> String {
    log::trace!("делаю текст");
    let patience = tg_contact.clone().patience.unwrap();
    let target = patience.target_string();
    let chain_text = patience.chain_to_string();
    let rest = patience.leftover_to_string();
    let dropout_text = patience.dropout_to_string();
    let text = format!(
        "### Цепочка\
        \n\
        \nЦелевая:\
        \n{}\
        \nПопытка: {}\
        \nКомбинация:\
        \n{}\
        \nОстаток: {}\
        \n\
        \n### Сведение\
        \n\
        \n{}",
        target, &patience.iteration, chain_text, rest, dropout_text
    );
    text
}

pub async fn spawn_patience_chain(
    bot: Bot,
    // msg: Message,
    dialoque: TeloxideDialogue,
    mut tg_contact: TgContact,
) -> Result<Message, TexoxideError> {
    log::trace!("спавн цепочки");
    let mut patience = tg_contact.clone().patience.unwrap();
    // dbg!(&patience.target);
    let text = make_text(tg_contact.clone());

    let keyboard = make_keyboard(tg_contact.clone());
    let message: Message = bot
        // .parse_mode(ParseMode::MarkdownV2)
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

    // кнопки управления при сведении
    let mut row = vec![];
    // кнопка отмены предыдущего действия
    let text = "↶";
    let callback_data = "abort_drop";
    let btn = InlineKeyboardButton::callback(text, callback_data);
    row.push(btn);

    // кнопка возврата в меню
    let text = "<<<";
    let callback_data = "back_to_menu";
    let btn = InlineKeyboardButton::callback(text, callback_data);
    row.push(btn);

    // кнопка ОК
    let text = "OK";
    let callback_data = "ok";
    let btn = InlineKeyboardButton::callback(text, callback_data);
    row.push(btn);

    keyboard.push(row);

    InlineKeyboardMarkup::new(keyboard)
}

pub async fn update_patience(
    bot: Bot,
    dialogue: TeloxideDialogue,
    mut tg_contact: TgContact,
) -> Result<(), TexoxideError> {
    log::trace!("Обновляю расклад");

    // собрать новуыю клавиатуру
    let keyboard = make_keyboard(tg_contact.clone());

    let mut patience = tg_contact.patience.clone().unwrap();
    let mut msg = patience.patience_msg.unwrap();

    let old_keyboard = msg.reply_markup().unwrap();
    let new_keyboard = &keyboard;

    let old_text = msg.text().clone().unwrap_or("...");
    let new_text = make_text(tg_contact.clone()).trim().to_string();

    // если изменения в клавиатуре, применить
    if old_keyboard != new_keyboard || old_text != new_text {
        log::trace!("New keyboard");
        let msg_id = msg.id;
        msg = bot
            .edit_message_text(dialogue.chat_id(), msg_id, new_text)
            .reply_markup(keyboard)
            .await?;
        patience.patience_msg = Some(msg.clone());
        tg_contact.patience = Some(patience.clone());
        dialogue
            .update(State::Patience {
                tg_contact: tg_contact.clone(),
            })
            .await?;
    }

    Ok(())
}
