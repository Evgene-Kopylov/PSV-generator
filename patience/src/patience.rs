use colored::*;
use rand::rngs::mock::StepRng;
use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;

use crate::settings::MAX_ITERATIONS;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Card {
    pub suit: Option<String>,
    pub rank: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Deck {
    pub suits: Vec<String>,
    pub nominals: Vec<String>,
    pub full_deck: Vec<Card>,
    pub current_deck: Vec<Card>,
}

// #[allow(dead_code)]
pub fn give_default() -> (Vec<String>, Vec<String>) {
    let suits: Vec<String> = vec!["☐", "L", "▲", "♡", "○"]
        .iter()
        .map(|c| c.to_string())
        .collect();
    let ranks: Vec<String> = vec![
        "T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛",
    ]
    .iter()
    .map(|c| c.to_string())
    .collect();
    (suits, ranks)
}

pub struct MySpread {
    deck: Deck,
}

impl Card {
    pub fn new<T>(suit: T, rank: T) -> Self
    where
    T: Into<String>, {
        Self {
            suit: Some(suit.into()),
            rank: Some(rank.into()),
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let mut v: Vec<char> = s.chars().collect();
        Some(Self {
            suit: Some(v.pop()?.to_string()),
            rank: Some(v.iter().collect()),
        })
    }

    pub fn update_rank<T: Into<String>>(&mut self, rank: T) {
        self.rank = Some(rank.into());
    }

    pub fn update_suit<T: Into<String>>(&mut self, suit: T) {
        self.suit = Some(suit.into());
    }

}

impl Deck {
    pub fn new(suits: Vec<String>, nominals: Vec<String>) -> Self {
        let mut full_deck: Vec<Card> = vec![];

        for s in &suits {
            for n in &nominals {
                let card = Card::new(s.clone(), n.clone());
                full_deck.push(card.clone());
            }
        }

        Self {
            suits,
            nominals,
            full_deck: full_deck.clone(),
            current_deck: full_deck,
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = StepRng::new(2, 13);
        let mut irs = Irs::default();
        let _ = irs.shuffle(&mut self.current_deck, &mut rng);
    }

    pub fn drain(&mut self, n: usize) -> Vec<Card> {
        self.current_deck.drain(..n).collect()
    }

    pub fn refresh_deck(&mut self) -> () {
        self.current_deck = self.full_deck.clone();
    }
}

impl MySpread {
    pub fn new(deck: Deck) -> Self {
        Self { deck }
    }

    pub fn perform_chain_operation(&self, chain: Vec<Card>) -> bool {
        let mut chain = chain.clone();
        let mut line: String = String::new();
        line += "\n### Сведение\n\n";
        line += &chain
            .clone()
            .iter()
            .map(|c| {
                format!(
                    "{}{}",
                    c.clone().rank.unwrap_or("_".to_string()),
                    c.clone().suit.unwrap_or("_".to_string())
                )
            })
            .collect::<Vec<_>>()
            .join("  ");
        line += "\n";

        let max = chain.len();

        for _ in 0..max {
            let current = chain.len();
            if current <= 2 {
                println!("{}", line);
                return true;
            }
            for j in 0..current - 2 {
                line += "   ";
                line += &" ".repeat(
                    chain[j]
                        .rank
                        .clone()
                        .unwrap_or("_".to_string())
                        .chars()
                        .count(),
                );

                if (&chain[j].suit == &chain[j + 2].suit) || (&chain[j].rank == &chain[j + 2].rank)
                {
                    line += &format!(
                        "{}{}\n",
                        chain[j + 1].rank.clone().unwrap_or("_".to_string()).blue(),
                        chain[j + 1]
                            .suit
                            .clone()
                            .unwrap_or("_".to_string())
                            .yellow()
                    );

                    chain.remove(j + 1);

                    line += &chain
                        .iter()
                        .map(|c| {
                            format!(
                                "{}{}",
                                c.clone().rank.unwrap_or("_".to_string()),
                                c.clone().suit.unwrap_or("_".to_string())
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("  ");
                    line += "\n";
                    break;
                }
            }
        }
        false
    }

    pub fn chain_check(&self, chain: Vec<Card>) -> bool {
        let mut chain = chain.clone();
        let max = chain.len();

        for _ in 0..max {
            let current = chain.len();
            if current <= 2 {
                return true;
            }
            for j in 0..current - 2 {
                if (&chain[j].suit == &chain[j + 2].suit) || (&chain[j].rank == &chain[j + 2].rank)
                {
                    chain.remove(j + 1);
                    break;
                }
            }
        }
        false
    }

    pub fn patience(&mut self, target: Vec<&str>) -> () {
        for i in 0..MAX_ITERATIONS {
            self.deck.refresh_deck();
            self.deck.shuffle();
            let mut target_chain = vec![];

            for item in &target {
                if item.chars().all(|c| c.is_digit(10)) {
                    let n: usize = item.parse().unwrap();
                    let part: Vec<Card> = self.deck.drain(n);
                    target_chain.extend(part);
                } else if let Some(card) = Card::from_str(item) {
                    target_chain.push(card);
                }
            }
            self.print_chain(target_chain.clone());

            if self.chain_check(target_chain.clone()) {
                self.perform_chain_operation(target_chain.clone());
                println!("Итерация: {:}", i);
                break;
            }
        }
    }

    fn print_chain(&self, chain: Vec<Card>) -> () {
        let mut line = String::new();
        for c in &chain {
            line += &c.clone().rank.unwrap_or("_".to_string());
            line += &c.clone().suit.unwrap_or("_".to_string());
            line += "  ";
        }
    }
}
