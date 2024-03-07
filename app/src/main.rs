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

use logging::logging_config;
use patience_lib::patience::give_default;

type TexoxideError = Box<dyn Error + Send + Sync>;
type TeloxideDialogue = Dialogue<State, InMemStorage<State>>;


#[derive(Clone)]
pub struct TgContact {
    suits: Vec<String>,
    ranks: Vec<String>,
}


impl TgContact {
    fn new() -> Self {
        let (suits, ranks) = give_default();
        Self {
            suits,
            ranks,
        }
    }
    fn update_suit(&mut self, index: usize, value: String) {
        self.suits[index] = value;
    }
}


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

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .enter_dialogue::<Update, InMemStorage<State>, State>()
        .branch(Update::filter_message().branch(dptree::case![State::Start].endpoint(start)))
        .branch(
            Update::filter_callback_query()
                .branch(dptree::case![State::Start].endpoint(unexpected_callback)),
        )
        .branch(
            Update::filter_callback_query()
                .branch(dptree::case![State::Menu { tg_contact }].endpoint(menu_buttons)),
        )
        .branch(Update::filter_message().endpoint(unexpected_text));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .build()
        .dispatch()
        .await;
}

async fn unexpected_text(bot: Bot, msg: Message) -> Result<(), TexoxideError> {
    let text = "¯\\_(ツ)_/¯";
    bot.send_message(msg.chat.id, text).await?;
    Ok(())
}

async fn unexpected_callback(bot: Bot, dialogue: TeloxideDialogue,) -> Result<(), TexoxideError> {
    let text = "(×﹏×) /start ?";
    bot.send_message(dialogue.chat_id(), text).await?;
    Ok(())
}
