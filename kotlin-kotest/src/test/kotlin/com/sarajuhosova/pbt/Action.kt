package com.sarajuhosova.pbt

abstract class Action<T> {

    abstract fun run(t: T): T

}
