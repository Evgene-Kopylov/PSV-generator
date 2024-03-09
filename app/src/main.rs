use teloxide::{
    dispatching::{dialogue::InMemStorage, UpdateFilterExt},
    prelude::*,
    types::Update,
};

mod menu_ui;
use menu_ui::start;

mod menu_buttons;
use menu_buttons::menu_buttons;

use dotenv::dotenv;
use std::error::Error;

use logging::logging_config;
use patience_lib::patience::{give_default, Card};

type TexoxideError = Box<dyn Error + Send + Sync>;
type TeloxideDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(Clone)]
pub struct TgContact {
    suits: Vec<String>,
    ranks: Vec<String>,
    chain: Vec<Option<Card>>,
    active_index: Option<usize>,
}

impl TgContact {
    fn new() -> Self {
        let (suits, ranks) = give_default();
        let mut chain = Vec::new();
        for _ in 0..10 {
            chain.push(None);
        }
        Self {
            suits,
            ranks,
            chain,
            active_index: None,
        }
    }
    fn update_suit<T, V>(&mut self, index: T, value: V)
    where
        T: Into<usize>,
        V: Into<String>,
    {
        self.suits[index.into()] = value.into();
    }

    fn chain_expend<T: Into<usize>>(&mut self, n: T) {
        for _ in 0..n.into() {
            if self.chain.len() >= 40 {
                return;
            }
            self.chain.push(None);
        }
    }

    fn chain_reduce(&mut self) {
        self.chain.pop();
    }

    fn update_chain<T>(&mut self, rank: Option<T>, suit: Option<T>)
    where
        T: Into<String> + Copy,
    {
        log::trace!(
            "update chain rank = {:?}, suit = {:?}",
            if let Some(r) = rank {
                let _ = r.into();
            },
            if let Some(s) = suit {
                let _ = s.into();
            },
        );
        if let Some(index) = self.active_index {
            if let Some(rank) = rank {
                if let Some(_card) = &self.chain[index] {
                    log::trace!("есть карта!!!");
                    if let Some(card) = self.chain.get_mut(index).unwrap() {
                        card.update_rank(rank);
                    }
                } else {
                    let card = Card::new(None, Some(rank));
                    self.chain[index] = Some(card);
                    log::trace!("Новая карта!")
                }
            }

            if let Some(suit) = suit {
                if let Some(_card) = &self.chain[index] {
                    log::trace!("есть карта!!!");
                    if let Some(card) = self.chain.get_mut(index).unwrap() {
                        card.update_suit(suit);
                    }
                } else {
                    let card = Card::new(Some(suit), None);
                    self.chain[index] = Some(card);
                    log::trace!("Новая карта!")
                }
            }
        }
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
