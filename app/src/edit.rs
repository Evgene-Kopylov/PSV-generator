// #![allow(unused)] // FIXME

// use crate::menu_buttons::update_menu;
use teloxide::{prelude::Bot, requests::Requester, types::Message};

use crate::{menu::buttons::update_menu, State, TeloxideDialogue, TexoxideError, TgContact};

pub async fn edit(
    bot: Bot,
    dialogue: TeloxideDialogue,
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
            update_menu(bot, dialogue, tg_contact).await?;
        }
    }
    Ok(())
}
