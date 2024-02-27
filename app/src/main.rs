#![allow(unused)]  // FIXME


struct Card {
    suit: String,
    nominal: String,
}

impl Card {
    fn new(suit: String, nominal: String) -> Self {
        Self {
            suit,
            nominal,
        }
    }
}


struct Deck {
    suits: Vec<String>,
    nominals: Vec<String>,
    deck: Vec<Card>,
}



impl Deck {
    fn new(
        nominals: Vec<&str>, 
        suits: Vec<&str>
    ) -> Self {
        let suits: Vec<String> = suits
            .iter()
            .map(|s| s
                .to_string())
            .collect();

        let nominals: Vec<String> = nominals
            .iter()
            .map(|s| s
                .to_string())
            .collect();

        let mut deck: Vec<Card> = vec![];
        for s in &suits {
            for n in &nominals {
                let card = Card::new(s.clone(), n.clone());
                deck.push(card);
            }
        }

        Self {
            suits,
            nominals,
            deck,
        }
    }

    fn psv(&self, target: Vec<&str>) -> () {
        // dbg!(target);
        for item in target {
            println!("{:}", item);
            if item.chars().all(|c| c.is_digit(10)) {
                println!("digit");
            } else {
                println!("not digit");
            }
        }
    }

}




fn main() {
    println!("_+_+_+_++_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_++_+_+_+_+_+_+_+_+");
    let suits = vec!["☐", "L", "▲", "♡", "○"];
    let nominal = vec!["T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛"];
    let mut deck = Deck::new(
        suits, 
        nominal
    );
    let target = vec!["2", "2○", "β☐", "4", "5L", "2"];
    deck.psv(target);
}
