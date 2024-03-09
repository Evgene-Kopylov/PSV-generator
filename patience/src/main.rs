#![allow(unused)]

use std::time::Instant;

mod patience;
mod settings;
mod test_patience;
use logging::logging_config;

use crate::patience::{give_default, Card, Deck, MySpread};

fn main() {
    std::env::set_var("LOG_LEVEL", "info,patience=trace");
    logging_config::logging_config("LOG_LEVEL");
    log::info!("Patience...");

    let start = Instant::now();
    // let suits = vec!["☐", "L", "▲", "♡", "○"];  // TODO use default()
    // let ranks = vec![
    // "T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛",
    // ];

    let (suits, ranks) = give_default();
    let mut deck = Deck::new(suits, ranks);
    let target = vec!["1", "2○", "β☐", "1"];

    let mut chain = vec![];
    for item in &target {
        if item.chars().all(|c| c.is_digit(10)) {
            let n: usize = item.parse().unwrap();
            for _ in 0..n {
                chain.push(None);
            }
        } else if let Some(card) = Card::from_str(item) {
            chain.push(Some(card));
        }
    }
    chain.push(Some(Card::new(None, Some("♛"))));
    chain.push(Some(Card::new(Some("☐"), None)));
    dbg!(&chain);
    let mut my_spread = MySpread::new(deck);
    my_spread.patience(chain);
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
