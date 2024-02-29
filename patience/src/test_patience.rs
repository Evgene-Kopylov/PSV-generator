#[cfg(test)]
use crate::patience::*;

#[test]
fn test_card_creation() {
    let card = Card::new("♡".to_string(), "A".to_string());
    assert_eq!(card.suit, "♡");
    assert_eq!(card.rank, "A");
}

#[test]
fn test_card_creation_from_str() {
    let Some(card) = Card::from_str("3♢") else {
        panic!();
    };
    assert_eq!(card.suit, "♢");
    assert_eq!(card.rank, "3");
}

#[test]
fn test_deck_creation() {
    let suits = vec!["☐", "L", "▲", "♡", "○"];
    let nominals = vec![
        "T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛",
    ];
    let deck = Deck::new(suits, nominals);
    assert_eq!(deck.suits.len(), 5);
    assert_eq!(deck.nominals.len(), 13);
    assert_eq!(deck.full_deck.len(), 65); // 5 suits * 13 nominals
    assert_eq!(deck.current_deck.len(), 65);
}

#[test]
fn test_deck_shuffle() {
    let suits = vec!["☐", "L", "▲", "♡", "○"];
    let nominals = vec![
        "T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛",
    ];
    let mut deck = Deck::new(suits, nominals);
    let original_order = deck.current_deck.clone();
    deck.shuffle();
    assert_ne!(deck.current_deck, original_order);
}

#[test]
fn test_deck_drain() {
    let suits = vec!["☐", "L", "▲", "♡", "○"];
    let nominals = vec![
        "T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛",
    ];
    let mut deck = Deck::new(suits, nominals);
    let drained_cards = deck.drain(5);
    assert_eq!(deck.current_deck.len(), 60);
    assert_eq!(drained_cards.len(), 5);
}

#[test]
fn test_deck_refresh() {
    let suits = vec!["☐", "L", "▲", "♡", "○"];
    let nominals = vec![
        "T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛",
    ];
    let mut deck = Deck::new(suits, nominals);
    deck.shuffle();
    deck.refresh_deck();
    assert_eq!(deck.current_deck.len(), 65);
}

#[test]
fn test_my_spread_chain_check() {
    let suits = vec!["☐", "L", "▲", "♡", "○"];
    let nominals = vec![
        "T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛",
    ];
    let deck = Deck::new(suits, nominals);
    let my_spread = MySpread::new(deck);
    let chain = vec![
        Card::new("☐".to_string(), "2".to_string()),
        Card::new("L".to_string(), "2".to_string()),
        Card::new("▲".to_string(), "2".to_string()),
    ];
    assert!(my_spread.chain_check(chain));
}

#[test]
fn test_my_spread_perform_chain_operation() {
    let suits = vec!["☐", "L", "▲", "♡", "○"];
    let nominals = vec![
        "T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛",
    ];
    let deck = Deck::new(suits, nominals);
    let my_spread = MySpread::new(deck);
    let chain = vec![
        Card::new("☐".to_string(), "2".to_string()),
        Card::new("L".to_string(), "2".to_string()),
        Card::new("▲".to_string(), "2".to_string()),
    ];
    assert!(my_spread.perform_chain_operation(chain));
}
