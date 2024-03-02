use teloxide::prelude::{Bot, CallbackQuery};

use crate::errors::HandlerResult;

pub async fn callback_handler(bot: Bot, q: CallbackQuery) -> HandlerResult {
    let callback_data = q.clone().data.unwrap_or_default();

    if let Some((category, value)) = split_callback_data(&callback_data) {
        match category {
            "rank" => handle_rank_callback(bot, q.clone(), value).await?,
            "suit" => handle_suit_callback(bot, q.clone(), value).await?,
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

async fn handle_rank_callback(bot: Bot, q: CallbackQuery, rank_value: &str) -> HandlerResult {
    // Handle rank callback, perform actions based on the rank value
    // ...
    println!("rank_  value = {}", rank_value);
    Ok(())
}

async fn handle_suit_callback(bot: Bot, q: CallbackQuery, suit_value: &str) -> HandlerResult {
    // Handle suit callback, perform actions based on the suit value
    // ...
    println!("suit_  value = {}", suit_value);
    Ok(())
}
