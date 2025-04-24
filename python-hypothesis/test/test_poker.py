from hypothesis import given, strategies as st
from src import poker
from pytest import raises

# Test that a deck has 52 cards
@given(st.builds(poker.Deck))
def test_deck_size(deck: poker.Deck):
    assert len(deck.cards) == 52
    
# Test that a deck has 52 unique cards
@given(st.builds(poker.Deck))
def test_deck_unique(deck: poker.Deck):
    assert len(deck.cards) == len(set(deck.cards))
    
# Test that all 52 cards are in the deck
@given(st.builds(poker.Deck))
def test_deck_all_cards(deck: poker.Deck):
    all_cards = [poker.Card(suit, rank) for suit in poker.Suit for rank in poker.Rank]
    assert len(deck.cards) == len(all_cards)
    for card in all_cards:
        assert card in deck.cards
    
# Test that dealing to x players where 1 <= x <= 10 results in x lists of 5 cards
@given(st.integers(min_value=1, max_value=10), st.builds(poker.Deck))
def test_deal_less_than_ten(num_players: int, deck: poker.Deck):
    hands = deck.deal(num_players)
    assert len(hands) == num_players
    for hand in hands:
        assert len(hand) == 5

# Test that dealing to < 1 players results in an error
@given(st.integers(max_value=0), st.builds(poker.Deck))
def test_deal_less_than_one(num_players: int, deck: poker.Deck):
    with raises(ValueError):
        deck.deal(num_players)
        
# Test that dealing to > 10 players results in an error
@given(st.integers(min_value=11), st.builds(poker.Deck))
def test_deal_greater_than_ten(num_players: int, deck: poker.Deck):
    with raises(ValueError):
        deck.deal(num_players)

# Test that dealing to x players leaves 52 - 5x cards in the deck
@given(st.integers(min_value=1, max_value=10), st.builds(poker.Deck))
def test_cards_left(num_players: int, deck: poker.Deck):
    deck.deal(num_players)
    assert len(deck.cards) == 52 - (5 * num_players)
    
# Test that each hand has 5 cards
@given(st.integers(min_value=1, max_value=10), st.builds(poker.Deck))
def test_hand_size(num_players: int, deck: poker.Deck):
    hands = deck.deal(num_players)
    for hand in hands:
        assert len(hand) == 5
        
# Test that each hand has unique cards
@given(st.integers(min_value=1, max_value=10), st.builds(poker.Deck))
def test_hand_unique(num_players: int, deck: poker.Deck):
    hands = deck.deal(num_players)
    for hand in hands:
        assert len(hand) == len(set(hand))
        
# Test that no hands share cards
@given(st.integers(min_value=1, max_value=10), st.builds(poker.Deck))
def test_hands_no_intersect(num_players: int, deck: poker.Deck):
    hands = deck.deal(num_players)
    for hand in hands:
        other_hands = hands.copy()
        other_hands.remove(hand)
        for other_hand in other_hands:
            assert len(set(hand) & set(other_hand)) == 0