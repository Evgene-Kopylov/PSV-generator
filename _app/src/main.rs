#![allow(unused)]

mod button_handler;
mod command_handler;
use button_handler::callback_handler;
use command_handler::{commands_handler, solitare_menu, Command};

use dotenv_codegen::dotenv;
use patience_lib::patience::MySpread;

use teloxide::{
    dispatching::{dialogue::InMemStorage, UpdateFilterExt},
    prelude::*,
    types::Update,
};

use std::error::Error;

type TexoxideError = Box<dyn Error + Send + Sync>;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Menu,
    ReceiveFullName,
    ReceiveAge {
        full_name: String,
    },
    ReceiveLocation {
        full_name: String,
        age: u8,
    },
}

#[tokio::main]
async fn main() {
    log::info!("Начало работы...");

    let bot = Bot::new(dotenv!("TELOXIDE_TOKEN"));

    let handler = dptree::entry()
        .enter_dialogue::<Update, InMemStorage<State>, State>()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(commands_handler),
        )
        .branch(Update::filter_message().branch(dptree::case![State::Menu].endpoint(solitare_menu)))
        .branch(Update::filter_message().endpoint(text_message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .build()
        .dispatch()
        .await;
}

async fn _start(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<(), TexoxideError> {
    bot.send_message(msg.chat.id, "Let's start! What's your full name?")
        .await?;
    dialogue.update(State::ReceiveFullName).await?;
    Ok(())
}

async fn text_message_handler(bot: Bot, msg: Message) -> Result<(), TexoxideError> {
    println!("text_message_handler");
    Ok(())
}
