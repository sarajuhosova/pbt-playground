package com.sarajuhosova.pbt.basket

import com.sarajuhosova.pbt.Action

class AddAction(
    private val product: Product,
    private val quantity: Int
) : Action<Basket>() {

    override fun run(t: Basket): Basket {
        t.addProduct(product, quantity)
        return t
    }

}
