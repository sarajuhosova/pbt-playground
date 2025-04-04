package com.sarajuhosova.pbt.minmax

import io.kotest.core.spec.style.StringSpec
import io.kotest.property.forAll
import kotlin.math.max
import kotlin.math.min

class MinMaxTest: StringSpec({

    "(1) min(a, a) is a" { forAll<Int> { a -> min(a, a) == a } }

    "(2) max(a, b) == max(b, a)" { forAll<Int, Int> { a, b -> max(a, b) == max(b, a) } }

    "(3) min(a, b) == a -> max(a, b) == b" {
        forAll<Int, Int> { a, b ->
            if (min(a, b) == a) max(a, b) == b else true
        }
    }

})
