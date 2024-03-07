#![allow(unused)] // FIXME

use std::ops::Index;

use log::trace;
use patience_lib::patience::Card;
use teloxide::{dispatching::dialogue::GetChatId, payloads::EditMessageReplyMarkupSetters, prelude::{Bot, CallbackQuery}, requests::Requester};

use crate::{start::{make_keyboard, spawn_menu}, State, TeloxideDialogue, TexoxideError, TgContact};

pub async fn menu_buttons(
    bot: Bot,
    dialogue: TeloxideDialogue,
    q: CallbackQuery,
    mut tg_contact: TgContact,
) -> Result<(), TexoxideError> {
    log::trace!("menu_buttons");
    let callback_data = q.clone().data.unwrap_or_default();
    dbg!(&tg_contact.chain);

    match callback_data {
        data if data.starts_with("rank") => handle_rank_callback(bot, dialogue, q.clone(), &data).await?,
        data if data.starts_with("suit") => {
                handle_suit_callback(
                    bot, 
                    dialogue, 
                    q.clone(), 
                    &data,
                    tg_contact).await?
            }
        data if data.starts_with("info") => { log::trace!("Информационная кнопка"); }
        data if data.starts_with("+") => handle_plus_btn(bot, dialogue, q.clone(), tg_contact).await?,
        
        _ => {
            log::debug!("Не определена категория");
        }
    }


    Ok(())
}


async fn handle_plus_btn(
    bot: Bot,
    dialogue: TeloxideDialogue,
    q: CallbackQuery,
    // data: &str,
    mut tg_contact: TgContact,
) -> Result<(), TexoxideError> {
    tg_contact.chain_extend();
    // dialogue.update(State::Menu { tg_contact }).await?;


    let keyboard = make_keyboard(tg_contact.clone());

    bot.edit_message_reply_markup(dialogue.chat_id(), q.message.unwrap()
        .id)
        .reply_markup(keyboard)
        .await?;
    dialogue.update(State::Menu { tg_contact }).await?;

    Ok(())
}

fn split_callback_data(data: &str) -> (&str, &str) {
    let parts: Vec<&str> = data.split('_').collect();
    if parts.len() == 2 {
        (parts[0], parts[1])
    } else {
        ("_", "_")
    }
}

async fn handle_rank_callback(
    bot: Bot,
    dialogue: TeloxideDialogue,
    q: CallbackQuery,
    data: &str,
) -> Result<(), TexoxideError> {
    // Handle rank callback, perform actions based on the rank value
    // ...
    let (_, rank) = split_callback_data(data);
    log::trace!("rank_value = {}", rank);
    Ok(())
}

async fn handle_suit_callback(
    bot: Bot,
    dialogue: TeloxideDialogue,
    q: CallbackQuery,
    data: &str,
    mut tg_contact: TgContact,
) -> Result<(), TexoxideError> {
    let (_, suit) = split_callback_data(data);
    log::trace!("suit_value = {}", suit);

    if let Some(index) = get_index_by_value( tg_contact.clone().suits, suit.to_string()) {
        tg_contact.update_suit(index, "__".to_string());

        let keyboard = make_keyboard(tg_contact.clone());

        bot.edit_message_reply_markup(dialogue.chat_id(), q.message.unwrap()
        .id)
        .reply_markup(keyboard).await?;
        dialogue.update(State::Menu { tg_contact }).await?;
    }

    Ok(())
}

fn get_index_by_value(v: Vec<String>, value: String) -> Option<usize> {
    if let Some(index) = v.iter().position(|x| x == &value) {
        log::trace!("Index of {} is: {}", value, index);
        Some(index)
    } else {
        log::trace!("Element {} not found in the vector", value);
        None
    }
}

fn modify_by_index(mut v: Vec<String>, index: usize, new_value: String) -> Vec<String> {
    if let Some(element) = v.iter_mut().nth(index) {
        *element = new_value;
        log::trace!("Modified vector: {:#?}", v);
    } else {
        log::trace!("Index {} is out of bounds", index);
    }
    v
}
