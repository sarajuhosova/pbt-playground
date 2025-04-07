// Warnings generated because code only used inside tests
#![allow(dead_code)]

use proptest::prelude::*;

#[cfg(test)]
use rand::{SeedableRng, rngs::StdRng, seq::SliceRandom};
#[cfg(test)]
use std::collections::HashSet;

proptest! {
  // A deck has 52 cards -> Verified by the type system because of the `FullDeck` type.
  // We instead test that it never crashes!
  #[test]
  fn test_01(seed: u64) {
    let mut deck: FullDeck = create_deck();

    // Shuffle
    let mut rng = StdRng::seed_from_u64(seed);
    deck.shuffle(&mut rng);
  }

  /// A deck has 52 unique cards
  #[test]
  fn test_02(seed: u64) {
    // Deck guaranteed to have 52 cards
    let mut deck = create_deck();

    // Shuffle
    let mut rng = StdRng::seed_from_u64(seed);
    deck.shuffle(&mut rng);

    // Test for uniqueness
    let mut seen_cards = HashSet::<Card>::new();
    for card in &deck {
      assert!(!seen_cards.contains(card));
      let _ = seen_cards.insert(card.clone());
    }
  }

  /// All 52 cards are in the deck (this verifies the same as above but with different semantics).
  #[test]
  fn test_03(seed: u64) {
    let mut deck: FullDeck = create_deck();

    // Shuffle
    let mut rng = StdRng::seed_from_u64(seed);
    deck.shuffle(&mut rng);

    // Add all combinations to list
    let mut list = Vec::<Card>::with_capacity(52);
    for rank in RANKS {
      for suit in SUITS {
        let card = Card { suit, rank };
        list.push(card);
      }
    }

    // For each card on deck
    for card in deck {
      // Find and remove from list
      let index = list.iter().position(|c| *c == card);
      let _ = list.remove(index.expect("Item exists"));
    }

    // List should be empty
    assert!(list.is_empty());
  }

  /// Dealing to X players where 1 <= X <= 10 results in X lists of 5 cards.
  #[test]
  fn test_04(seed: u64, x in 1u32..=10u32) {
    let mut deck = create_deck().to_vec();

    // Shuffle
    let mut rng = StdRng::seed_from_u64(seed);
    deck.shuffle(&mut rng);

    let hands = deal(x, &mut deck).expect("X is valid");

    // X lists
    assert!(hands.len() == x as usize);
    for sublist in hands {
      // Of 5 cards
      assert!(sublist.len() == 5);
    }
  }

  /// Dealing to X players where X < 1 results in an error.
  ///
  /// Note: Because `deal` is defined for u32s, the only possible value below 1 is 0.
  #[test]
  fn test_05(seed: u64) {
    let mut deck = create_deck().to_vec();

    // Shuffle
    let mut rng = StdRng::seed_from_u64(seed);
    deck.shuffle(&mut rng);

    // Test that deal for 0 returns an error
    let res = deal(0, &mut deck);

    assert!(res.is_err());
  }

  /// Dealing to X players where X > 10 results in an error.
  #[test]
  fn test_06(seed: u64, x in 11u32..) {
    let mut deck = create_deck().to_vec();

    // Shuffle
    let mut rng = StdRng::seed_from_u64(seed);
    deck.shuffle(&mut rng);

    // Test that deal for > 10 returns an error
    let res = deal(x, &mut deck);

    assert!(res.is_err());
  }

  /// Dealing to X players leaves 52 - 5X cards in the deck.
  #[test]
  fn test_07(seed: u64, x in 1u32..=10u32) {
    let mut deck = create_deck().to_vec();

    // Shuffle
    let mut rng = StdRng::seed_from_u64(seed);
    deck.shuffle(&mut rng);

    let _hands = deal(x, &mut deck).expect("X is valid");

    assert!(deck.len() == 52 - 5 * x as usize);
  }

  /// Each hand has 5 cards.
  #[test]
  fn test_08(seed: u64, x in 1u32..=10u32) {
    let mut deck = create_deck().to_vec();

    // Shuffle
    let mut rng = StdRng::seed_from_u64(seed);
    deck.shuffle(&mut rng);

    let hands = deal(x, &mut deck).expect("X is valid");

    // Each hand
    for hand in hands {
      // Has 5 cards
      assert!(hand.len() == 5);
    }
  }

  /// Each hand has 5 unique cards.
  #[test]
  fn test_09(seed: u64, x in 1u32..=10u32) {
    let mut deck = create_deck().to_vec();

    // Shuffle
    let mut rng = StdRng::seed_from_u64(seed);
    deck.shuffle(&mut rng);

    let hands = deal(x, &mut deck).expect("X is valid");

    // Each hand
    for hand in &hands {
      // has 5 cards
      assert!(hand.len() == 5);

      let mut seen_cards = HashSet::<Card>::new();

      // Make sure the cards in the hands are unique
      for card in hand {
        assert!(!seen_cards.contains(card));
        let _ = seen_cards.insert(card.clone());
      }
      assert!(seen_cards.len() == 5);
    }
  }

  /// No hands shared cards.
  #[test]
  fn test_10(seed: u64, x in 1u32..=10u32) {
    let mut deck = create_deck().to_vec();

    // Shuffle
    let mut rng = StdRng::seed_from_u64(seed);
    deck.shuffle(&mut rng);

    let hands = deal(x, &mut deck).expect("X is valid");
    let mut seen_cards = HashSet::<Card>::new();

    // Each hand
    for hand in &hands {
      // Has 5 cards
      assert!(hand.len() == 5);

      // Each card is unique
      for card in hand {
        assert!(!seen_cards.contains(card));
        let _ = seen_cards.insert(card.clone());
      }
    }

    // Sanity check: deck + cards of all hands should be 52.
    // We already checked that each hand has 5 cards.
    assert!(deck.len() + hands.len() * 5 == 52);
  }
}

#[derive(Debug)]
enum PokerError {
  ZeroPlayers,
  MoreThanTenPlayers,
}

type FullDeck = [Card; 52];

fn deal(players: u32, deck: &mut Vec<Card>) -> Result<Vec<Vec<Card>>, PokerError> {
  if players == 0 {
    return Err(PokerError::ZeroPlayers);
  }

  if players > 10 {
    return Err(PokerError::MoreThanTenPlayers);
  }

  let players = players as usize;

  let mut lists = Vec::<Vec<Card>>::with_capacity(players);

  let dealt = &deck[..players * 5];

  for i in 0..players {
    let player_split = dealt[i * 5..(i + 1) * 5].to_vec();
    lists.push(player_split);
  }

  let _ = deck.drain(..players * 5);

  Ok(lists)
}

/// Generates an *un*-shuffled deck of cards.
fn create_deck() -> FullDeck {
  SUITS
    .into_iter()
    .flat_map(|suit| {
      RANKS
        .into_iter()
        .map(move |rank| Card { suit, rank })
        .collect::<Vec<Card>>()
    })
    .collect::<Vec<_>>()
    .try_into()
    .expect("Size is 52")
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Card {
  suit: Suit,
  rank: Rank,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Suit {
  Spades,
  Diamonds,
  Clubs,
  Hearts,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Rank {
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
  Ace,
}

const SUITS: [Suit; 4] = [Suit::Spades, Suit::Diamonds, Suit::Clubs, Suit::Hearts];
const RANKS: [Rank; 13] = [
  Rank::Two,
  Rank::Three,
  Rank::Four,
  Rank::Five,
  Rank::Six,
  Rank::Seven,
  Rank::Eight,
  Rank::Nine,
  Rank::Ten,
  Rank::Jack,
  Rank::Queen,
  Rank::King,
  Rank::Ace,
];
