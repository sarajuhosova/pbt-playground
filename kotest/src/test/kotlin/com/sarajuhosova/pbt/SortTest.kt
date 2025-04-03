package com.sarajuhosova.pbt

import com.sarajuhosova.pbt.poker.Deck
import io.kotest.core.spec.style.StringSpec
import io.kotest.property.Arb
import io.kotest.property.arbitrary.*
import io.kotest.property.forAll

class SortTest : StringSpec({
    fun <T> listEqual(left: List<T>, right: List<T>): Boolean {
        if (left.size != right.size) return false
        for (i in left.indices) {
            if (left[i] != right[i]) return false
        }
        return true
    }

    fun <T: Comparable<T>> compareSorts(input: List<T>): Boolean =
        listEqual(input.insertionSorted(), input.sorted())

    "custom insertion sort does the same as built-in sort on integers" {
        forAll(Arb.list(Arb.int())) { list -> compareSorts(list) }
    }

    "custom insertion sort does the same as built-in sort on doubles" {
        forAll(Arb.list(Arb.double())) { list -> compareSorts(list) }
    }

    "custom insertion sort does the same as built-in sort on strings" {
        forAll(Arb.list(Arb.string())) { list -> compareSorts(list) }
    }

    data class Example(val a: Int, val b: String) : Comparable<Example> {
        override fun compareTo(other: Example): Int {
            val aa = this.a.compareTo(other.a)
            return if (aa != 0) aa else this.b.compareTo(other.b)
        }
    }

    fun arbitraryExample(): Arb<Example> = arbitrary {
        Example(Arb.int().bind(), Arb.string().bind())
    }

    "custom insertion sort does the same as built-in sort on data classes" {
        forAll(Arb.list(arbitraryExample())) { list -> compareSorts(list) }
    }

})
