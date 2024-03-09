use teloxide::{
    payloads::EditMessageReplyMarkupSetters,
    prelude::{Bot, CallbackQuery},
    requests::Requester,
};

use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

use crate::TeloxideDialogue;
use crate::TexoxideError;
use crate::{State, TgContact};

use patience_lib::patience::{Deck, MySpread};

pub async fn patience(
    _bot: Bot,
    dialogue: TeloxideDialogue,
    _q: CallbackQuery,
    tg_contact: TgContact,
) -> Result<(), TexoxideError> {
    Ok(())
}

pub async fn spawn_patience_chain(
    bot: Bot,
    // msg: Message,
    tg_contact: TgContact,
) -> Result<Message, TexoxideError> {
    let text = "### Сведение";

    let keyboard = make_keyboard(tg_contact.clone());
    let message: Message = bot
        // .parse_mode(ParseMode::Html)
        .send_message(tg_contact.menu_msg.unwrap().chat.id, text)
        .reply_markup(keyboard)
        .await?;
    Ok(message)
}

pub fn make_keyboard(tg_contact: TgContact) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    // дополнить список рангов до кратной числу кнопок в ряду длянны.
    let btn_row_size = 5;
    let mut chain = tg_contact.clone().patience.unwrap();
    // let reminder = btn_row_size - chain.len() % btn_row_size;
    // if chain.len() % btn_row_size > 0 && reminder > 0 {
    //     chain.extend(std::iter::repeat(" ".to_string()).take(reminder));
    // }

    // // Информационная кнопка
    // let row = vec![InlineKeyboardButton::callback("Ранги", "info_ranks")];
    // keyboard.push(row);

    // // грид кнопок рангов
    // for rank in chain.chunks(btn_row_size) {
    //     let row = rank
    //         .iter()
    //         .map(|item| InlineKeyboardButton::callback(item, "rank_".to_owned() + &item))
    //         .collect();

    //     keyboard.push(row);
    // }
    InlineKeyboardMarkup::new(keyboard)
}
