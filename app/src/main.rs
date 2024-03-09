use teloxide::{
    dispatching::{dialogue::InMemStorage, UpdateFilterExt},
    prelude::*,
    types::Update,
};

mod menu_ui;
mod tg_contact;
use menu_ui::start;

mod menu_buttons;
use menu_buttons::menu_buttons;

mod edit;
use edit::edit;

use dotenv::dotenv;
use std::error::Error;
use tg_contact::TgContact;

use logging::logging_config;

type TexoxideError = Box<dyn Error + Send + Sync>;
type TeloxideDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    Menu {
        tg_contact: TgContact,
    },
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    logging_config::logging_config("LOG_LEVEL");

    log::info!("Начало работы...");

    let bot = Bot::from_env(); //.parse_mode(ParseMode::Html);

    let handler = dptree::entry()
        .enter_dialogue::<Update, InMemStorage<State>, State>()
        // Start
        .branch(Update::filter_message().branch(dptree::case![State::Start].endpoint(start)))
        .branch(
            Update::filter_callback_query()
                .branch(dptree::case![State::Start].endpoint(unexpected_callback)),
        )
        // Menu
        .branch(
            Update::filter_callback_query()
                .branch(dptree::case![State::Menu { tg_contact }].endpoint(menu_buttons)),
        )
        .branch(
            Update::filter_message()
                .branch(dptree::case![State::Menu { tg_contact }].endpoint(edit)),
        )
        // Не к месту.
        .branch(Update::filter_message().endpoint(unexpected_text_message))
        .endpoint(unexpected_update);

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .build()
        .dispatch()
        .await;
}

async fn unexpected_text_message(bot: Bot, msg: Message) -> Result<(), TexoxideError> {
    let text = "¯\\_(ツ)_/¯";
    bot.send_message(msg.chat.id, text).await?;
    Ok(())
}

async fn unexpected_callback(
    bot: Bot,
    dialogue: TeloxideDialogue,
    _q: CallbackQuery,
) -> Result<(), TexoxideError> {
    let text = "(ᵔ.ᵔ) /start ?";
    bot.send_message(dialogue.chat_id(), text).await?;
    Ok(())
}

async fn unexpected_update(
    update: Update,
    bot: Bot,
    dialogue: TeloxideDialogue,
) -> Result<(), TexoxideError> {
    let text = "(×﹏×)";
    log::warn!("Update не распознан. {:#?}", update);
    bot.send_message(dialogue.chat_id(), text).await?;
    Ok(())
}
