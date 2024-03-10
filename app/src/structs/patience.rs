use teloxide::prelude::*;

use patience_lib::patience::Card;

#[derive(Debug, Clone)]
pub struct Patience {
    /// целевая последовательность. содержит не определенные элементы
    pub target: Vec<Option<Card>>,

    /// сложившаяся цепочка. полная
    pub chain: Vec<Card>,

    /// остаток сложения
    pub leftover: Vec<Card>,

    /// итерация, на которой пасьянс сложился
    pub iteration: usize,

    /// карты в полядке выбывания
    pub backlog: Vec<Card>,

    /// сообщение с гридом кнопок
    pub patience_msg: Option<Message>,
}

impl Patience {
    pub fn new(
        target: Vec<Option<Card>>,
        chain: Vec<Card>,
        leftover: Vec<Card>,
        iteration: usize,
    ) -> Self {
        Self {
            target,
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

    fn _vec_to_string(&self, vec: Vec<Card>) -> String {
        vec.iter()
            .map(|card| {
                format!(
                    "{}{}",
                    card.clone().rank.unwrap_or("_".to_string()),
                    card.clone().suit.unwrap_or("_".to_string()),
                )
            })
            .collect::<Vec<_>>()
            .join("  ")
    }

    pub fn chain_to_string(&self) -> String {
        self._vec_to_string(self.chain.clone())
    }

    pub fn leftover_to_string(&self) -> String {
        self._vec_to_string(self.leftover.clone())
    }

    pub fn target_string(&self) -> String {
        log::trace!("get target string representation");
        self.target
            .iter()
            .map(|c| {
                if let Some(card) = c {
                    format!(
                        "{}{}",
                        card.clone().rank.unwrap_or("_".to_string()),
                        card.clone().suit.unwrap_or("_".to_string()),
                    )
                } else {
                    "__".to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("   ")
    }
}
