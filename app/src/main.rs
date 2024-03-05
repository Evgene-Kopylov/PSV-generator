use teloxide::{
    dispatching::{dialogue::InMemStorage, UpdateFilterExt},
    prelude::*,
    types::Update,
};

mod start;
use start::start;

mod menu_buttons;
use menu_buttons::menu_buttons;

use dotenv::dotenv;
use std::error::Error;

use env_logger;

use std::io::Write;

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
    dotenv().ok();

    env_logger::Builder::new()
    .format(|buf, record| {
        writeln!(
            buf,
            "{}: {}    {}:{}    {}",
            record.level(),
            record.args(),
            record.file().unwrap_or("unknown"),
            record.line().unwrap_or(0),
            chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
        )
    })
    .parse_env("RUST_LOG") 
    .init();

    log::debug!("DEBUG");
    log::info!("INFO");
    log::warn!("WARN");
    log::error!("ERROR");
    log::info!("Начало работы...");

    let bot = Bot::from_env();

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
