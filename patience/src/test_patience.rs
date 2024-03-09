#[cfg(test)]
use crate::patience::*;

#[test]
fn test_card_creation() {
    let card = Card::new("♡".to_string(), "A".to_string());
    assert_eq!(card.suit.unwrap(), "♡".to_string());
    assert_eq!(card.rank.unwrap(), "A".to_string());
}

#[test]
fn test_card_creation_from_str() {
    let card = Card::from_str("3♢").unwrap();
    assert_eq!(card.suit.unwrap(), "♢".to_string());
    assert_eq!(card.rank.unwrap(), "3".to_string());
}

#[test]
fn test_deck_creation() {
    let suits = vec!["☐", "L", "▲", "♡", "○"]
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();
    let nominals = vec![
        "T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛",
    ]
    .iter()
    .map(|&n| n.to_string())
    .collect::<Vec<String>>();
    let deck = Deck::new(suits.clone(), nominals.clone());
    assert_eq!(deck.suits.len(), suits.len());
    assert_eq!(deck.nominals.len(), nominals.len());
    assert_eq!(deck.full_deck.len(), suits.len() * nominals.len());
    assert_eq!(deck.current_deck.len(), suits.len() * nominals.len());
}

#[test]
fn test_deck_shuffle() {
    let suits = vec!["☐", "L", "▲", "♡", "○"]
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();
    let nominals = vec![
        "T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛",
    ]
    .iter()
    .map(|&n| n.to_string())
    .collect::<Vec<String>>();
    let mut deck = Deck::new(suits.clone(), nominals.clone());
    let original_order = deck.current_deck.clone();
    deck.shuffle();
    assert_ne!(deck.current_deck, original_order);
}

#[test]
fn test_deck_drain() {
    let suits = vec!["☐", "L", "▲", "♡", "○"]
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();
    let nominals = vec![
        "T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛",
    ]
    .iter()
    .map(|&n| n.to_string())
    .collect::<Vec<String>>();
    let mut deck = Deck::new(suits.clone(), nominals.clone());
    let drained_cards = deck.drain(5);
    assert_eq!(deck.current_deck.len(), suits.len() * nominals.len() - 5);
    assert_eq!(drained_cards.len(), 5);
}

#[test]
fn test_deck_refresh() {
    let suits = vec!["☐", "L", "▲", "♡", "○"]
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();
    let nominals = vec![
        "T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛",
    ]
    .iter()
    .map(|&n| n.to_string())
    .collect::<Vec<String>>();
    let mut deck = Deck::new(suits.clone(), nominals.clone());
    deck.shuffle();
    deck.refresh_deck();
    assert_eq!(deck.current_deck.len(), suits.len() * nominals.len());
}

#[test]
fn test_my_spread_chain_check() {
    let suits = vec!["☐", "L", "▲", "♡", "○"]
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();
    let nominals = vec![
        "T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛",
    ]
    .iter()
    .map(|&n| n.to_string())
    .collect::<Vec<String>>();
    let deck = Deck::new(suits.clone(), nominals.clone());
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
    let suits = vec!["☐", "L", "▲", "♡", "○"]
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();
    let nominals = vec![
        "T", "2", "3", "4", "5", "6", "7", "8", "9", "10", "β", "λ", "♛",
    ]
    .iter()
    .map(|&n| n.to_string())
    .collect::<Vec<String>>();
    let deck = Deck::new(suits.clone(), nominals.clone());
    let my_spread = MySpread::new(deck);
    let chain = vec![
        Card::new("☐".to_string(), "2".to_string()),
        Card::new("L".to_string(), "2".to_string()),
        Card::new("▲".to_string(), "2".to_string()),
    ];
    assert!(my_spread.perform_chain_operation(chain));
}
