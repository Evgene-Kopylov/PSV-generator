// #![allow(unused)] // FIXME

use crate::menu_buttons::update_menu;
use std::{fmt::Display, usize};
use teloxide::{
    payloads::EditMessageReplyMarkupSetters,
    prelude::{Bot, CallbackQuery},
    requests::Requester,
    types::Message,
};

use crate::{menu_ui::make_keyboard, State, TeloxideDialogue, TexoxideError, TgContact};

use patience_lib::patience::{Deck, MySpread};

pub async fn edit(
    bot: Bot,
    dialogue: TeloxideDialogue,
    // q: CallbackQuery,
    msg: Message,
    mut tg_contact: TgContact,
) -> Result<(), TexoxideError> {
    log::trace!("возможно редактирование");
    let _ = bot.delete_message(msg.chat.id, msg.id).await?;

    if let Some(index) = tg_contact.suit_index {
        log::trace!("suit_index = {}", index);
        if let Some(value) = msg.text() {
            log::trace!("msg.text = {}", value);
            tg_contact.update_suit(index, value);
            tg_contact.suit_index = None;
            dialogue
                .update(State::Menu {
                    tg_contact: tg_contact.clone(),
                })
                .await?;
            update_menu(bot, dialogue, msg, tg_contact).await?;
            // log::trace!("выбрана новая масть suit = {}", msg.text().unwrap());
        }
    }
    Ok(())
}
