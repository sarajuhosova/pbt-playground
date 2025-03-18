package com.sarajuhosova.pbt.poker

import com.sarajuhosova.pbt.poker.enums.Rank
import com.sarajuhosova.pbt.poker.enums.Suit
import com.sarajuhosova.pbt.poker.exception.TooManyPlayersException
import io.kotest.core.spec.style.StringSpec
import io.kotest.matchers.collections.shouldBeIn
import io.kotest.matchers.shouldBe
import io.kotest.property.Arb
import io.kotest.property.Exhaustive
import io.kotest.property.arbitrary.arbitrary
import io.kotest.property.arbitrary.int
import io.kotest.property.checkAll
import io.kotest.property.exhaustive.exhaustive
import io.kotest.property.exhaustive.flatMap
import io.kotest.property.exhaustive.ints
import io.kotest.property.exhaustive.map
import io.kotest.property.forAll

class DeckTest: StringSpec({

    fun arbitraryDeck(): Arb<Deck> = arbitrary { Deck() }

    fun suits(): Exhaustive<Suit> = Suit.entries.exhaustive()
    fun ranks(): Exhaustive<Rank> = Rank.entries.exhaustive()
    fun cards(): Exhaustive<Card> = suits().flatMap { suit ->
        ranks().map { rank -> Card(rank, suit) }
    }

    "(1) a deck has 52 cards" {
        forAll(arbitraryDeck()) { d -> d.size() == 52 }
    }

    "(2) a deck has 52 unique cards" {
        forAll(arbitraryDeck()) { d -> d.toList().toSet().size == 52 }
    }

    "(3) all 52 cards are in the deck" {
        val deck = Deck()
        checkAll(cards()) { card -> card shouldBeIn deck.toList() }
    }

    "(4) dealing to X players results in X hands" {
        checkAll(Exhaustive.ints(1..10)) { players ->
            val deck = Deck()
            val hands = deck.deal(players)
            hands.size shouldBe players
        }
    }

    "(5) cannot deal to less than 1 player" {
        forAll(Arb.int(max = 0)) { players ->
            val deck = Deck()
            try {
                deck.deal(players)
                false
            } catch (e: IllegalArgumentException) {
                true
            }
        }
    }

    "(6) dealing to more than 10 players throws an exception" {
        forAll(Arb.int(min = 11)) { players ->
            val deck = Deck()
            try {
                deck.deal(players)
                false
            } catch (e: TooManyPlayersException) {
                true
            }
        }
    }

    "(7) dealing to X players leaves 52 - 5X cards" {
        forAll(arbitraryDeck(), Arb.int(1, 10)) { deck, players ->
            deck.deal(players)
            deck.size() == (52 - (5 * players))
        }
    }

    "(8) each hand has 5 cards" {
        forAll(arbitraryDeck(), Arb.int(1, 10)) { deck, players ->
            val hands = deck.deal(players)
            hands.all { it.size == 5 }
        }
    }

    "(9) each hand has 5 unique cards" {
        forAll(arbitraryDeck(), Arb.int(1, 10)) { deck, players ->
            val hands = deck.deal(players)
            hands.all { it.toSet().size == 5 }
        }
    }

    "(10) two hands don't share cards" {
        forAll(arbitraryDeck(), Arb.int(1, 10)) { deck, players ->
            val hands = deck.deal(players)
            var unique = true
            for (i in hands.indices) {
                for (j in (i + 1) until hands.size) {
                    unique = unique && (hands[i].toSet() intersect hands[j].toSet()).isEmpty()
                }
            }
            unique
        }
    }

})
