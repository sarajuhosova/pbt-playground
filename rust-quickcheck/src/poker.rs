#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    #[derive(Clone, Debug)]
    enum Suit {
        Spades,
        Diamonds,
        Clubs,
        Hearts
    }

    #[derive(Clone, Debug)]
    struct Card {
        suit: Suit,
        rank: u32 // 1 = ace, 11 = jack, 12 = queen, 13 = king
    }

    fn deal(players: i32) -> Result<Vec<Vec<Card>>, String> {
        if players < 1 { return Err(String::from("Not enough players")) }
        if players > 10 { return Err(String::from("Too many players")) }

        let mut cards: Vec<Card> = Vec::with_capacity(52);
        for i in 1..=13 {
            cards.push(Card { suit: Suit::Spades, rank: i });
            cards.push(Card { suit: Suit::Diamonds, rank: i });
            cards.push(Card { suit: Suit::Clubs, rank: i });
            cards.push(Card { suit: Suit::Hearts, rank: i });
        }

        let mut result: Vec<Vec<Card>> = Vec::with_capacity(players as usize);
        for i in 0..players as usize {
            result.push(cards[i*5 .. (i+1)*5].to_vec())
        }
        Ok(result)
    }

    #[quickcheck]
    fn less_than_one_error(players: i32) -> bool {
        if players >= 1 { return true }
        if let Err(_) = deal(players) { true } else { false }
    }

    #[quickcheck]
    fn more_than_ten_error(players: i32) -> bool {
        if players <= 10 { return true }
        if let Err(_) = deal(players) { true } else { false }
    }

    #[quickcheck]
    fn correct_number_of_results(players: i32) -> bool {
        if players < 1 || players > 10 { return true }
        let result = deal(players).unwrap();
        if result.len() as i32 != players { return false }
        for hand in result { if hand.len() != 5 { return false } }
        true
    }
}
