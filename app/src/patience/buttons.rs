use teloxide::{
    prelude::{Bot, CallbackQuery},
    requests::Requester,
};

use crate::TexoxideError;
use crate::{patience::ui::update_patience, TeloxideDialogue};
use crate::{State, TgContact};
use teloxide::types::ParseMode;
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

use patience_lib::patience::{Card, Deck, MySpread};

pub async fn patience_solving(
    bot: Bot,
    dialogue: TeloxideDialogue,
    q: CallbackQuery,
    tg_contact: TgContact,
) -> Result<(), TexoxideError> {
    let callback_data = q.clone().data.unwrap_or_default();
    log::trace!("Patience btn. data = {}", &callback_data);

    match callback_data {
        data if data.starts_with("card") => {
            hendle_card_button(bot, dialogue, &data, tg_contact).await?;
        }
        _ => {
            log::debug!("Не определена категория");
        }
    }

    Ok(())
}

async fn hendle_card_button(
    bot: Bot,
    dialogue: TeloxideDialogue,
    data: &str,
    mut tg_contact: TgContact,
) -> Result<(), TexoxideError> {
    let parts: Vec<&str> = data.split('_').collect();
    let index = parts[1].parse::<usize>().unwrap();
    log::trace!("index = {}", index);
    let mut patience = tg_contact.patience.clone().unwrap();
    // dbg!(&patience);
    patience = patience.from_chain_to_backlog(index);
    tg_contact.patience = Some(patience);

    // if let Some(card) = patience.chain.get(index) {

    // tg_contact.from_chain_to_backlog(index);
    // tg_contact.patience.chain.remove(index);
    // tg_contact.patience.backlog.push(card.clone());

    dialogue
        .update(State::Patience {
            tg_contact: tg_contact.clone(),
        })
        .await?;
    update_patience(bot, dialogue, tg_contact).await?;
    Ok(())
}
