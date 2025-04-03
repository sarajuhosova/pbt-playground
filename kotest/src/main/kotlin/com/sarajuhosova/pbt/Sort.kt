package com.sarajuhosova.pbt

fun <T : Comparable<T>> List<T>.insertionSorted(): List<T> {
    if (this.isEmpty()) return emptyList()
    val sortedList = mutableListOf<T>()
    for (item in this) {
        var index = 0
        while (index < sortedList.size && sortedList[index] < item) {
            index++
        }
        sortedList.add(index, item)
    }
    return sortedList
}
