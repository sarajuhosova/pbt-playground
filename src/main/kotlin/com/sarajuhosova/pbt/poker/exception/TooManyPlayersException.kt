package com.sarajuhosova.pbt.poker.exception

class TooManyPlayersException(val players: Int): Exception("The maximum amount of players is 10")
