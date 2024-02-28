#![allow(unused)]  // FIXME

use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;



#[derive(Debug, Default, Clone, PartialEq)]
struct Card {
    suit: String,
    nominal: String,
}


#[derive(Debug, PartialEq, Clone)]
struct Deck {
    suits: Vec<String>,
    nominals: Vec<String>,
    new_deck: Vec<Card>,
    deck: Vec<Card>,
}


struct MySpread {
    deck: Deck,
    // target_chain: Vec<Card>,
}


impl Card {
    fn new(suit: String, nominal: String) -> Self {
        Self {
            suit,
            nominal,
        }
    }

    fn from_str(s: &str) -> Self {
        let mut v: Vec<char> = s.chars().collect();
        Self {
            suit: v.pop().unwrap().to_string(),
            nominal: v.iter().collect(),
        }
    }
}




impl Deck {
    fn new(
        suits: Vec<&str>,
        nominals: Vec<&str>, 
    ) -> Self {
        let suits: Vec<String> = suits
            .iter()
            .map(|s| s.to_string())
            .collect();

        let nominals: Vec<String> = nominals
            .iter()
            .map(|n| n.to_string())
            .collect();

        let mut new_deck: Vec<Card> = vec![];
        for s in &suits {
            for n in &nominals {
                let card = Card::new(s.clone(), n.clone());
                new_deck.push(card.clone());
                // println!("{:}{:}", &card.suit, &card.nominal);
            }
        }

        Self {
            suits,
            nominals,
            new_deck: new_deck.clone(),
            deck: new_deck,
        }
    }

    fn shuffle(&mut self) {

        let mut rng = StepRng::new(2, 13);
        let mut irs = Irs::default();
        
        
        let _ = irs.shuffle(&mut self.deck, &mut rng);
    }

    fn drain(&mut self, n: usize) -> Vec<Card> {
        self.deck.drain(..n).collect()
    }


    fn pop_card(&mut self, card: Card) -> Card {
        self.deck.remove(11);
        card
    }

    fn refresh_deck(&mut self) -> () {
        self.deck = self.new_deck.clone();
    }

}


impl MySpread {
    fn new(deck: Deck) -> Self {
        Self {
            deck,
            // target_chain: vec![],
        }
    }

    fn chain_check(&self, chain: Vec<Card>) -> bool {
        for i in 0..chain.len() {
            if chain.len() == 2 {
                return true;
            }
        }
        false
    }

    fn patience(&mut self, target: Vec<&str>) -> () {
        
        for i in 0..3000 {
            self.deck.refresh_deck();

            self.deck.shuffle();
            let mut target_chain = vec![];

            for item in &target {
                if item.chars().all(|c| c.is_digit(10)) {
                    let n: usize = item.parse().unwrap();
                    let part: Vec<Card> = self.deck.drain(n);
                    target_chain.extend(part);
                } else {
                    let card = Card::from_str(item);
                    self.deck.pop_card(card.clone());
                    target_chain.push(card);
                }
            }
            self.print_chain(target_chain.clone());
            
            if self.chain_check(target_chain.clone()) {
                println!("Итерация: {:}", i);
                break;
            }


        }



    }

    fn print_chain(&self, chain: Vec<Card>) -> () {
        let mut line = String::new();
        for c in &chain {
            line += &c.nominal;
            line += &c.suit;
            line += "  ";
            
        }
        // dbg!(line);
        let _line = chain.iter()
        .map(
            |c| format!("{}{}", c.nominal, c.suit)
        ).collect::<Vec<_>>()
        .join("  ");
        // dbg!(_line);
    }
}


fn main() {
    println!("_+_+_+_++_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_++_+_+_+_+_+_+_+_+");
    let suits = vec!["☐", "L", "▲", "♡", "○"];
    let nominal = vec!["T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛"];
    let deck = Deck::new(
        suits, 
        nominal
    );
    let target = vec!["2", "2○", "β☐", "4", "5L", "2"];
    let mut my_spread = MySpread::new(deck);
    my_spread.patience(target);
}
