use teloxide::{dispatching::dialogue, prelude::*};

use patience_lib::patience::{give_default, Card};

#[derive(Debug, Clone)]
pub struct Patience {
    pub chain: Vec<Card>,
    pub leftover: Vec<Card>,
    pub iteration: usize,
    pub backlog: Vec<Card>,
    pub patience_msg: Option<Message>,
}

impl Patience {
    pub fn new(chain: Vec<Card>, leftover: Vec<Card>, iteration: usize) -> Self {
        Self {
            chain,
            leftover,
            iteration,
            backlog: vec![],
            patience_msg: None,
        }
    }

    /// Перемещает карту по индексу из расклада в беклог.
    pub fn from_chain_to_backlog(&mut self, index: usize) -> Self {
        log::trace!("from chain to backlog");
        if index < self.chain.len() {
            let card = self.chain[index].clone();
            self.chain.remove(index);
            self.backlog.push(card.clone());
        }
        self.to_owned()
    }
}
