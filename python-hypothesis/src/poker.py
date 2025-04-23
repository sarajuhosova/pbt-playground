from enum import Enum
import random
from typing import List

class Suit(Enum):
    SPADES = "Spades"
    DIAMONDS = "Diamonds"
    CLUBS = "Clubs"
    HEARTS = "Hearts"

class Rank(Enum):
    TWO = "2"
    THREE = "3"
    FOUR = "4"
    FIVE = "5"
    SIX = "6"
    SEVEN = "7"
    EIGHT = "8"
    NINE = "9"
    TEN = "10"
    JACK = "Jack"
    QUEEN = "Queen"
    KING = "King"
    ACE = "Ace"

class Card:
    def __init__(self, suit: Suit, rank: Rank):
        self.suit = suit
        self.rank = rank

    def __repr__(self):
        return f"{self.rank.value} of {self.suit.value}"
    
    def __eq__(self, other):
        if not isinstance(other, Card):
            return NotImplemented
        return self.suit == other.suit and self.rank == other.rank

    def __hash__(self):
        return hash((self.suit, self.rank))

class Deck:
    def __init__(self):
        self.cards : List[List[Card]] = [Card(suit, rank) for suit in Suit for rank in Rank]
        self.shuffle()
        
    def __eq__(self, other):
        if not isinstance(other, Deck):
            return NotImplemented
        return set(self.cards) == set(other.cards)

    def shuffle(self):
        random.shuffle(self.cards)

    def deal(self, num_players: int) -> List[List[Card]]:
        if num_players < 1:
            raise ValueError("Must have at least one player to play!")
        if num_players * 5 > len(self.cards):
            raise ValueError("Not enough cards to deal!")
        
        hands : List[List[Card]] = []
        
        # Deal 5 cards to each player
        for _ in range(num_players):
            hand = self.cards[:5]
            hands.append(hand)
            self.cards = self.cards[5:]
        return hands
