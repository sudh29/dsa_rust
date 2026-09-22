# quick sort
import random


def partition(a, start, end):
    # random pivot to improve time complexity
    r = random.randint(start, end)

    a[r], a[end] = a[end], a[r]
    pivot = a[end]
    pindex = start
    for i in range(start, end):
        if a[i] < pivot:
            a[i], a[pindex] = a[pindex], a[i]
            pindex += 1
    a[pindex], a[end] = a[end], a[pindex]
    return pindex


def quickSort(a, start, end):
    if start < end:
        pindex = partition(a, start, end)
        quickSort(a, start, pindex - 1)
        quickSort(a, pindex + 1, end)


x = [5, 22, -6, 7, 2, 1, 0, 3]
(quickSort(x, 0, len(x) - 1))
print(x)


# Smaller code
def quicksort(array):
    if len(array) <= 1:
        return array

    pivot = array[len(array) // 2]
    smaller = [x for x in array if x < pivot]
    larger = [x for x in array if x >= pivot]

    return quicksort(smaller) + [pivot] + quicksort(larger)
