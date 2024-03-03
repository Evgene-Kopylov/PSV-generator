use teloxide::{
    dispatching::{dialogue::InMemStorage, UpdateFilterExt},
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Update},
};

mod start;
use start::start;

use dotenv_codegen::dotenv;
use std::error::Error;

type TexoxideError = Box<dyn Error + Send + Sync>;
type TeloxideDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    Menu,
}

#[tokio::main]
async fn main() {
    log::info!("Начало работы...");

    let bot = Bot::new(dotenv!("TELOXIDE_TOKEN"));

    let handler = dptree::entry()
        .enter_dialogue::<Update, InMemStorage<State>, State>()
        .branch(Update::filter_message().branch(dptree::case![State::Start].endpoint(start)));
    // .branch(Update::filter_callback_query().branch(dptree::case![State::Menu].endpoint(menu_buttons_handler)));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .build()
        .dispatch()
        .await;
}
