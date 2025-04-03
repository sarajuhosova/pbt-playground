# Property-Based Testing Playground

This is a playground for property-based testing.
The exercises are meant to help you get to know a PBT framework
    and can be repeated with any language / framework combination.
This repository currently contains solutions in:
* [`kotest` with Kotlin](./kotest)

## Exercise 1: `min` and `max`

To kick things off, let's begin with a simple integer comparison test.
Implement the two functions (or use the one provided in a library):
* `min(Int, Int)` which returns the smaller of two integers and
* `max(Int, Int)` which returns the larger  of two integers.

Verify the following properties:
1. Given any integer `a`, `min(a, a)` should return `a`.
2. Given any two integers `a` and `b`, `max(a, b)` should equal `max(b, a)`.
3. Given any two integers `a` and `b`, if  `min(a, b)` returns `a`, then `max(a, b)` should return `b`.

## Exercise 2: Reverse

Next, implement the holy grail of property-based testing examples: reversing a list.
You can either implement your own reverse method, or use a library one.

Verify the following properties:
1. Reversing a list with one element results in the same list.
2. Reversing a list twice results in the original list.
3. The length of a reversed list is the same as the length of the original list.

Try populating the generated lists with different types of elements for each property.

## Exercise 3: Sort = Sort?

Choose two types of sort algorithms (e.g., merge sort and quick sort), and verify that they always produce the same output.
You can implement one or both on your own, or use library implementations.

## Exercise 4: Poker

*inspired by: https://johanneslink.net/property-based-testing-in-kotlin/#the-poker-domain*

First, define a data structure for a deck of cards.
A deck consists of 52 cards, each card having
    a suit (Spades, Diamonds, Clubs, Hearts) and
    a rank (2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace).

Next define, a `deal(players: Int): [[Card]]` function on this data structure which
* takes the number of players dealing the cards and
* returns a list of lists of cards, where each list of cards represents the 5 cards dealt to a player.

Verify the following properties:
1. A deck has 52 cards.
2. A deck has 52 *unique* cards.
3. All 52 cards are in the deck *(this verifies the same as above but with different semantics)*.
4. Dealing to `X` players where `1 <= X <= 10` results in `X` lists of 5 cards.
5. Dealing to `X` players where `X < 1` results in an error.
6. Dealing to `X` players where `X > 10` results in an error.
7. Dealing to `X` players leaves `52 - 5X` cards in the deck.
8. Each hand has 5 cards.
9. Each hand has 5 *unique* cards.
10. No hands shared cards.
