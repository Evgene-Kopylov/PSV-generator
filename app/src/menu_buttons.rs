// #![allow(unused)] // FIXME

use std::{fmt::Display, usize};

use teloxide::{
    payloads::EditMessageReplyMarkupSetters,
    prelude::{Bot, CallbackQuery},
    requests::Requester,
};

use crate::{menu_ui::make_keyboard, State, TeloxideDialogue, TexoxideError, TgContact};

pub async fn menu_buttons(
    bot: Bot,
    dialogue: TeloxideDialogue,
    q: CallbackQuery,
    tg_contact: TgContact,
) -> Result<(), TexoxideError> {
    log::trace!("menu_buttons");
    let callback_data = q.clone().data.unwrap_or_default();

    match callback_data {
        data if data.starts_with("rank") => {
            handle_rank_callback(bot, dialogue, q.clone(), &data, tg_contact).await?
        }
        data if data.starts_with("suit") => {
            handle_suit_callback(bot, dialogue, q.clone(), &data, tg_contact).await?
        }
        data if data.starts_with("info") => {
            log::trace!("Информационная кнопка");
        }
        data if data.starts_with("+") => {
            handle_plus_btn(bot, dialogue, q.clone(), tg_contact).await?
        }
        data if data.starts_with("-") => {
            handle_minus_btn(bot, dialogue, q.clone(), tg_contact).await?
        }
        data if data.starts_with("chain") => {
            handle_select_in_chain(bot, dialogue, q.clone(), tg_contact).await?
        }
        _ => {
            log::debug!("Не определена категория");
        }
    }

    Ok(())
}

async fn handle_select_in_chain(
    bot: Bot,
    dialogue: TeloxideDialogue,
    q: CallbackQuery,
    mut tg_contact: TgContact,
) -> Result<(), TexoxideError> {
    let data = q.data.clone().unwrap_or(String::new());
    let parts: Vec<&str> = data.split('_').collect();
    if parts.len() == 2 {
        let index = parts[1].parse::<usize>().unwrap();
        log::trace!("active_index = {}", &index);
        tg_contact.active_index = Some(index);
        dialogue
            .update(State::Menu {
                tg_contact: tg_contact.clone(),
            })
            .await?;
    }

    update_menu(bot, dialogue, q, tg_contact).await?;
    Ok(())
}

async fn handle_minus_btn(
    bot: Bot,
    dialogue: TeloxideDialogue,
    q: CallbackQuery,
    mut tg_contact: TgContact,
) -> Result<(), TexoxideError> {
    tg_contact.chain_reduce();

    update_menu(bot, dialogue, q, tg_contact).await?;

    Ok(())
}

async fn update_menu(
    bot: Bot,
    dialogue: TeloxideDialogue,
    q: CallbackQuery,
    tg_contact: TgContact,
) -> Result<(), TexoxideError> {
    let keyboard = make_keyboard(tg_contact.clone());

    let old_msg = &q.message.clone().unwrap();
    let old_keyboard = old_msg.reply_markup().unwrap();
    let new_keyboard = &keyboard;
    if old_keyboard != new_keyboard {
        bot.edit_message_reply_markup(dialogue.chat_id(), q.message.unwrap().id)
            .reply_markup(keyboard)
            .await?;
        dialogue.update(State::Menu { tg_contact }).await?;
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
    let data = q.data.clone().unwrap_or(String::new());
    log::trace!("btn {}", data);
    let count = data
        .strip_prefix("+")
        .unwrap_or("0")
        .parse::<usize>()
        .unwrap_or(0);

    tg_contact.chain_expend(count);

    update_menu(bot, dialogue, q, tg_contact).await?;

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
    mut tg_contact: TgContact,
) -> Result<(), TexoxideError> {
    let (_, rank) = split_callback_data(data);
    log::trace!("rank_value = {}", rank);

    tg_contact.update_chain(Some(rank), None);

    dialogue
        .update(State::Menu {
            tg_contact: tg_contact.clone(),
        })
        .await?;

    update_menu(bot, dialogue, q, tg_contact).await?;
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

    if let Some(index) = get_index_by_value(tg_contact.clone().suits, suit) {
        tg_contact.update_suit(index, "__".to_string());

        let keyboard = make_keyboard(tg_contact.clone());

        bot.edit_message_reply_markup(dialogue.chat_id(), q.message.unwrap().id)
            .reply_markup(keyboard)
            .await?;
        dialogue.update(State::Menu { tg_contact }).await?;
    }

    Ok(())
}

fn get_index_by_value<T>(v: Vec<String>, value: T) -> Option<usize>
where
    T: Into<String> + Display,
{
    let value = value.into();
    if let Some(index) = v.iter().position(|x| x == &value) {
        log::trace!("Index of {} is: {}", value, index);
        Some(index)
    } else {
        log::trace!("Element {} not found in the vector", value);
        None
    }
}
