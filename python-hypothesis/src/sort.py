from typing import List

def insertion_sort(xs:List):
    lst = xs
    n = len(lst)
    
    if n <= 1:
        return lst
    
    for i in range(1, n):
        elem = lst[i]
        j = i-1
        while j >= 0 and elem < lst[j]:
            lst[j+1] = lst[j]
            j -= 1
        lst[j+1] = elem
    
    return lst

    
def merge(lst1: List, lst2: List) -> List:
    result = []
    i = j = 0

    while i < len(lst1) and j < len(lst2):
        if lst1[i] < lst2[j]:
            result.append(lst1[i])
            i += 1
        else:
            result.append(lst2[j])
            j += 1

    result.extend(lst1[i:])
    result.extend(lst2[j:])
    return result

def merge_sort(xs: List) -> List:
    if len(xs) <= 1:
        return xs

    mid = len(xs) // 2
    left = merge_sort(xs[:mid])
    right = merge_sort(xs[mid:])
    return merge(left, right)
    
        