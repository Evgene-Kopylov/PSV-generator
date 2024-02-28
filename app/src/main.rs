#![allow(unused)]  // FIXME

use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;
use std::io::{repeat, Read};
use std::time::Instant;
use colored::*;

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
        let (matched, remaining): (Vec<Card>, Vec<Card>) = self.deck.clone()
            .into_iter()
            .partition(|c| *c == card.clone()); 
        self.deck = remaining;
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


    fn chain_fold_droaw(&self, chain: Vec<Card>) -> bool {
        let mut chain = chain.clone();
        let mut line: String = String::new();
        line += "\n### Сведение\n\n";
        line += &chain.iter()
            .map(|c| format!("{}{}", c.nominal, c.suit))
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
                line += &" ".repeat(chain[j].nominal.chars().count());


                if (&chain[j].suit == &chain[j+2].suit) || (&chain[j].nominal == &chain[j+2].nominal) {
                    line += &format!("{}{}\n", chain[j+1].nominal.blue(), chain[j+1].suit.yellow());
                    
                    chain.remove(j+1);

                    line += &chain.iter()
                        .map(|c| format!("{}{}", c.nominal, c.suit))
                        .collect::<Vec<_>>()
                        .join("  ");
                    line += "\n";
                    break;
                }
            }
        }
        false
    }


    fn chain_check(&self, chain: Vec<Card>) -> bool {
        let mut chain = chain.clone();
        
        let max = chain.len();

        for _ in 0..max {
            let current = chain.len();
            if current <= 2 {
                return true;
            }
            for j in 0..current - 2 {
                                if (&chain[j].suit == &chain[j+2].suit) || (&chain[j].nominal == &chain[j+2].nominal) {
                                        chain.remove(j+1);
                    break;
                }
            }
        }
        false
    }

    fn patience(&mut self, target: Vec<&str>) -> () {
        
        for i in 0..10000 {
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
                    let pop = self.deck.pop_card(card.clone());
                    // assert!(card == pop.unwrap());
                    target_chain.push(card);
                }
            }
            self.print_chain(target_chain.clone());
            
            if self.chain_check(target_chain.clone()) {
                self.chain_fold_droaw(target_chain.clone());
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
    let start = Instant::now();
    let suits = vec!["☐", "L", "▲", "♡", "○"];
    let nominal = vec!["T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛"];
    let deck = Deck::new(
        suits, 
        nominal
    );
    // let target = vec!["40", "2○", "β☐", "2☐", "3○"];
    let target = vec!["4", "2○", "β☐", "2☐", "3○", "9"];
    let mut my_spread = MySpread::new(deck);
    my_spread.patience(target);
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
