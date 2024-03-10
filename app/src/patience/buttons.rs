use teloxide::prelude::{Bot, CallbackQuery};

use crate::TexoxideError;
use crate::{patience::ui::update_patience, TeloxideDialogue};
use crate::{State, TgContact};

/// Обработка кнопок на стадии сведения пасьянса.
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

/// Обработка нажатий на кнопки - карты при сложении пасьянса.
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
    patience = patience.from_chain_to_backlog(index);
    tg_contact.patience = Some(patience);

    dialogue
        .update(State::Patience {
            tg_contact: tg_contact.clone(),
        })
        .await?;
    update_patience(bot, dialogue, tg_contact).await?;
    Ok(())
}
