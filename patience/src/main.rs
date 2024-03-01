use std::time::Instant;

mod patience;
mod settings;
mod test_patience;
use crate::patience::{Deck, MySpread};

fn main() {
    let start = Instant::now();
    let suits = vec!["☐", "L", "▲", "♡", "○"];
    let ranks = vec![
        "T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛",
    ];
    let deck = Deck::new(suits, nominals);
    let target = vec!["4", "2○", "β☐", "2☐", "3○", "9"];
    let mut my_spread = MySpread::new(deck);
    my_spread.patience(target);
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
