#![allow(unused)] // FIXME

use teloxide::prelude::{Bot, CallbackQuery};

use crate::{TeloxideDialogue, TexoxideError};

pub async fn menu_buttons(
    bot: Bot,
    dialogue: TeloxideDialogue,
    q: CallbackQuery,
) -> Result<(), TexoxideError> {
    let callback_data = q.clone().data.unwrap_or_default();

    if let Some((category, value)) = split_callback_data(&callback_data) {
        match category {
            "rank" => handle_rank_callback(bot, dialogue, q.clone(), value).await?,
            "suit" => handle_suit_callback(bot, dialogue, q.clone(), value).await?,
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
    suit_value: &str,
) -> Result<(), TexoxideError> {
    // Handle suit callback, perform actions based on the suit value
    // ...
    println!("suit_  value = {}", suit_value);
    Ok(())
}
