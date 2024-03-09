use teloxide::{
    prelude::{Bot, CallbackQuery},
    requests::Requester,
};

use crate::TeloxideDialogue;
use crate::TexoxideError;
use crate::{State, TgContact};
use teloxide::types::ParseMode;
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

use patience_lib::patience::{Deck, MySpread};

pub async fn patience_solving(
    bot: Bot,
    dialogue: TeloxideDialogue,
    q: CallbackQuery,
    tg_contact: TgContact,
) -> Result<(), TexoxideError> {
    Ok(())
}
