#![allow(unused)] // FIXME

use std::ops::Index;

use log::trace;
use teloxide::{dispatching::dialogue::GetChatId, payloads::EditMessageReplyMarkupSetters, prelude::{Bot, CallbackQuery}, requests::Requester};

use crate::{start::{make_keyboard, spawn_menu}, State, TeloxideDialogue, TexoxideError};

pub async fn menu_buttons(
    bot: Bot,
    dialogue: TeloxideDialogue,
    suits: Vec<String>,
    q: CallbackQuery,
) -> Result<(), TexoxideError> {
    log::trace!("menu_buttons");
    let callback_data = q.clone().data.unwrap_or_default();

    if let Some((category, value)) = split_callback_data(&callback_data) {
        match category {
            "rank" => handle_rank_callback(bot, dialogue, q.clone(), value).await?,
            "suit" => {
                handle_suit_callback(bot, dialogue, q.clone(), value.to_string(), suits).await?
            }
            _ => {
                log::debug!("Unknown category, handle accordingly or ignore");
            }
        }
    } else {
        log::debug!("{:#?}", &callback_data);
    }

    Ok(())
}

fn split_callback_data(data: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = data.split('_').collect();
    if parts.len() == 2 {
        Some((parts[0], parts[1]))
    } else {
        None
    }
}

async fn handle_rank_callback(
    bot: Bot,
    dialogue: TeloxideDialogue,
    q: CallbackQuery,
    rank_value: &str,
) -> Result<(), TexoxideError> {
    // Handle rank callback, perform actions based on the rank value
    // ...
    log::trace!("rank_value = {}", rank_value);
    Ok(())
}

async fn handle_suit_callback(
    bot: Bot,
    dialogue: TeloxideDialogue,
    q: CallbackQuery,
    suit_value: String,
    suits: Vec<String>,
) -> Result<(), TexoxideError> {
    log::trace!("suit_value = {}", suit_value);

    if let Some(index) = get_index_by_value(suits.clone(), suit_value) {
        let suits = modify_by_index(suits, index, "__".to_string());
        let ranks = vec![  // fixme
            "T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛",
        ];

        let keyboard = make_keyboard(suits.iter().map(|c| c.as_str()).collect(), ranks);

        bot.edit_message_reply_markup(dialogue.chat_id(), q.message.unwrap()
        .id)
        .reply_markup(keyboard).await?;
        dialogue.update(State::Menu { suits }).await?;
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
