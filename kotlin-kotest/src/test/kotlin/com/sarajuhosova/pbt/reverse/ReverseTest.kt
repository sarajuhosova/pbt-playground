package com.sarajuhosova.pbt.reverse

import io.kotest.core.spec.style.StringSpec
import io.kotest.property.Arb
import io.kotest.property.arbitrary.double
import io.kotest.property.arbitrary.list
import io.kotest.property.arbitrary.string
import io.kotest.property.forAll

class ReverseTest : StringSpec({
    "(1) reverse single is itself" {
        forAll<Int> { i ->
            listOf(i).reversed() == listOf(i)
        }
    }

    "(2) reverse reverse is itself" {
        forAll(Arb.list(Arb.string())) { list ->
            list.reversed().reversed() == list
        }
    }

    "(3) length of reverse is the same" {
        forAll(Arb.list(Arb.double())) { list ->
            list.reversed().size == list.size
        }
    }
})
