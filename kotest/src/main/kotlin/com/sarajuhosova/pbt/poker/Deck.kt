package com.sarajuhosova.pbt.poker

import com.sarajuhosova.pbt.poker.enums.Rank
import com.sarajuhosova.pbt.poker.enums.Suit
import com.sarajuhosova.pbt.poker.exception.EmptyDeckException
import com.sarajuhosova.pbt.poker.exception.TooManyPlayersException

class Deck {

    private val cards =
        Rank.entries.flatMap { rank ->
            Suit.entries.map { suit ->
                Card(rank, suit)
            }
        }.shuffled().toMutableList()

    fun size() = cards.size

    private fun deal(): Card {
        if (cards.isEmpty()) throw EmptyDeckException()
        return cards.removeAt(0)
    }

    fun deal(players: Int): List<List<Card>> {
        if (players < 1) throw IllegalArgumentException("There must be at least one player")
        if (players > 10) throw TooManyPlayersException(players)
        val hands = List(players) { mutableListOf<Card>() }
        repeat(5) { for (player in hands) player.add(deal()) }
        return hands
    }

    fun toList(): List<Card> = cards.toList()

}
