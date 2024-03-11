use teloxide::prelude::*;

use patience_lib::patience::Card;

#[derive(Debug, Clone)]
pub struct Patience {
    /// целевая последовательность. содержит не определенные элементы
    pub target: Vec<Option<Card>>,

    /// сложившаяся цепочка. полная
    pub chain: Vec<Card>,

    /// цепочка без изменений
    pub _chain_base: Vec<Card>,

    /// остаток сложения
    pub leftover: Vec<Card>,

    /// итерация, на которой пасьянс сложился
    pub iteration: usize,

    /// карты в полядке выбывания
    pub backlog: Vec<Card>,

    /// порядок сложения карт пользователем
    pub dropout_order: Vec<usize>,

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
            chain: chain.clone(),
            _chain_base: chain,
            leftover,
            iteration,
            backlog: vec![],
            dropout_order: vec![],
            patience_msg: None,
        }
    }

    pub fn drop_card(&mut self, card: Card) {
        log::trace!(
            "выбывание карты {}{}",
            &card.rank.clone().unwrap(),
            &card.suit.clone().unwrap()
        );

        if let Some(base_index) = self._chain_base.iter().position(|x| x == &card) {
            self.dropout_order.push(base_index);

            if let Some(chain_index) = self.chain.iter().position(|x| x == &card) {
                self.chain.remove(chain_index);
            }
        }
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
        self._vec_to_string(self._chain_base.clone())
    }

    pub fn leftover_to_string(&self) -> String {
        self._vec_to_string(self.leftover.clone())
    }

    /// Строка с картами в порядке сложения пользователем
    pub fn dropout_to_string(&self) -> String {
        let mut s = String::new();
        for i in &self.dropout_order {
            if let Some(card) = self._chain_base.clone().get(*i) {
                s += &format!(
                    "  {}{}",
                    card.clone().rank.unwrap_or("_".to_string()),
                    card.clone().suit.unwrap_or("_".to_string()),
                );
            } else {
                log::error!("Ошибка при записи порядка выбывания.");
            }
        }
        log::trace!("выбывание: {}", &s);
        s
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
