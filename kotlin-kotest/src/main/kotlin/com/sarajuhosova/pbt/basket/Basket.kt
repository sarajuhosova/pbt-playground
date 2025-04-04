package com.sarajuhosova.pbt.basket

class Basket(private val stock: Map<Product, Int>) {

    private val products = stock.mapValues { 0 }.toMutableMap()

    fun addProduct(product: Product, quantity: Int) {
        if (quantity <= 0) {
            throw IllegalArgumentException("Quantity must be greater than zero")
        }
        if (product !in products || quantity + (products[product] ?: 0) > (stock[product] ?: 0)) {
            throw IllegalArgumentException("Product not available")
        }
        products[product] = (products[product] ?: 0) + quantity
    }

    fun setProductQuantity(product: Product, quantity: Int) {
        if (quantity < 0) {
            throw IllegalArgumentException("Quantity must be non-negative")
        }
        if (product !in products || quantity > (stock[product] ?: 0)) {
            throw IllegalArgumentException("Product not available")
        }
        products[product] = quantity
    }

    fun removeProduct(product: Product) {
        if (product !in products) {
            throw IllegalArgumentException("Product not available")
        }
        products[product] = 0
    }

}
