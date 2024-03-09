/// стартовое сообщение
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

use crate::TexoxideError;
use crate::{menu::ui::spawn_menu, TeloxideDialogue};
use crate::{State, TgContact};

pub async fn start(
    bot: Bot,
    dialoque: TeloxideDialogue,
    msg: Message,
) -> Result<(), TexoxideError> {
    log::trace!("Start");
    let mut tg_contact = TgContact::new();
    let menu_message = spawn_menu(bot, msg, tg_contact.clone()).await?;
    tg_contact.menu_msg = Some(menu_message);
    dialoque.update(State::Menu { tg_contact }).await?;
    log::trace!("Произошел спавн меню.");
    Ok(())
}
