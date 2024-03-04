// use patience_lib::patience::Card;
use teloxide::{
    dispatching::{dialogue::InMemStorage, UpdateFilterExt},
    prelude::*,
    types::Update,
};

mod start;
use start::start;

mod menu_buttons;
use menu_buttons::menu_buttons;

use dotenv_codegen::dotenv;
use std::error::Error;

type TexoxideError = Box<dyn Error + Send + Sync>;
type TeloxideDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    Menu {
        suits: Vec<String>,
    },
}

#[tokio::main]
async fn main() {
    log::info!("Начало работы...");

    let bot = Bot::new(dotenv!("TELOXIDE_TOKEN"));

    let handler = dptree::entry()
        .enter_dialogue::<Update, InMemStorage<State>, State>()
        .branch(Update::filter_message().branch(dptree::case![State::Start].endpoint(start)))
        .branch(
            Update::filter_callback_query()
                .branch(dptree::case![State::Menu { suits }].endpoint(menu_buttons)),
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .build()
        .dispatch()
        .await;
}
