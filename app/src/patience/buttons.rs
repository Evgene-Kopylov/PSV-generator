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
        data if data.starts_with("abort") => {
            hendle_abort_last_action(bot, dialogue, tg_contact).await?;
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

    if let Some(mut patience) = tg_contact.patience.clone() {
        if let Some(card) = patience.chain.get(index) {
            patience.drop_card(card.clone());
            tg_contact.patience = Some(patience);

            dialogue
                .update(State::Patience {
                    tg_contact: tg_contact.clone(),
                })
                .await?;
            update_patience(bot, dialogue, tg_contact).await?;
        }
    }

    Ok(())
}

async fn hendle_abort_last_action(
    bot: Bot,
    dialogue: TeloxideDialogue,
    mut tg_contact: TgContact,
) -> Result<(), TexoxideError> {
    log::trace!("abort btn");

    if let Some(mut patience) = tg_contact.patience.clone() {
        patience.abort_drop();
        tg_contact.patience = Some(patience);

        dialogue
            .update(State::Patience {
                tg_contact: tg_contact.clone(),
            })
            .await?;
        update_patience(bot, dialogue, tg_contact).await?;
    }

    Ok(())
}
