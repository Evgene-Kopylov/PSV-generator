use teloxide::prelude::*;

use patience_lib::patience::{give_default, Card};

#[derive(Clone)]
pub struct TgContact {
    pub suits: Vec<String>,
    pub ranks: Vec<String>,
    pub chain: Vec<Option<Card>>,
    pub chain_index: Option<usize>,
    pub suit_index: Option<usize>,
    pub menu_msg: Option<Message>,
    pub patience: Option<Vec<Card>>,
}

impl TgContact {
    pub fn new() -> Self {
        let (suits, ranks) = give_default();
        let mut chain = Vec::new();
        for _ in 0..10 {
            chain.push(None);
        }
        Self {
            suits,
            ranks,
            chain,
            chain_index: None,
            suit_index: None,
            menu_msg: None,
            patience: None,
        }
    }
    pub fn update_suit<T, V>(&mut self, index: T, value: V)
    where
        T: Into<usize>,
        V: Into<String>,
    {
        self.suits[index.into()] = value.into();
    }

    pub fn chain_expend<T: Into<usize>>(&mut self, n: T) {
        for _ in 0..n.into() {
            if self.chain.len() >= 40 {
                return;
            }
            self.chain.push(None);
        }
    }

    pub fn chain_reduce(&mut self) {
        self.chain.pop();
    }

    pub fn update_chain<T>(&mut self, rank: Option<T>, suit: Option<T>)
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
        if let Some(index) = self.chain_index {
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
