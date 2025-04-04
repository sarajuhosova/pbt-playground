package com.sarajuhosova.pbt.basket

import io.kotest.property.Arb
import io.kotest.property.arbitrary.arbitrary
import io.kotest.property.arbitrary.int
import io.kotest.property.arbitrary.list
import io.kotest.property.arbitrary.string

fun arbitraryProduct(): Arb<Product> = arbitrary {
    Product(Arb.string().bind())
}

fun arbitraryStock(): Arb<Map<Product, Int>> = arbitrary {
    Arb.list(arbitraryProduct()).bind()
        .distinct()
        .associateWith { Arb.int(min = 1).bind() }
}
