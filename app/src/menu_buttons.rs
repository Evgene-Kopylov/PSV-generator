#![allow(unused)] // FIXME

use std::ops::Index;

use teloxide::prelude::{Bot, CallbackQuery};

use crate::{start::spawn_menu, State, TeloxideDialogue, TexoxideError};

pub async fn menu_buttons(
    bot: Bot,
    dialogue: TeloxideDialogue,
    suits: Vec<String>,
    q: CallbackQuery,
) -> Result<(), TexoxideError> {
    let callback_data = q.clone().data.unwrap_or_default();

    if let Some((category, value)) = split_callback_data(&callback_data) {
        match category {
            "rank" => handle_rank_callback(bot, dialogue, q.clone(), value).await?,
            "suit" => {
                handle_suit_callback(bot, dialogue, q.clone(), value.to_string(), suits).await?
            }
            _ => {
                println!("Unknown category, handle accordingly or ignore");
            }
        }
    } else {
        dbg!(&callback_data);
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
    println!("rank_  value = {}", rank_value);
    Ok(())
}

async fn handle_suit_callback(
    bot: Bot,
    dialogue: TeloxideDialogue,
    q: CallbackQuery,
    suit_value: String,
    suits: Vec<String>,
) -> Result<(), TexoxideError> {
    println!("suit_  value = {}", suit_value);

    if let Some(index) = get_index_by_value(suits.clone(), suit_value) {
        let suits = modify_by_index(suits, index, "__".to_string());
        dialogue.update(State::Menu { suits }).await?;
    }

    Ok(())
}

fn get_index_by_value(v: Vec<String>, value: String) -> Option<usize> {
    if let Some(index) = v.iter().position(|x| x == &value) {
        log::info!("Index of {} is: {}", value, index);
        let suits = modify_by_index(v, index, "__".to_string());
        Some(index)
    } else {
        log::info!("Element {} not found in the vector", value);
        None
    }
}

fn modify_by_index(mut v: Vec<String>, index: usize, new_value: String) -> Vec<String> {
    if let Some(element) = v.iter_mut().nth(index) {
        *element = new_value;
        log::info!("Modified vector: {:?}", v);
    } else {
        log::info!("Index {} is out of bounds", index);
    }
    v
}
