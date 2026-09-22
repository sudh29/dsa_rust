# function to sort a list using selction sort


def selectionSort(a):
    n = len(a)
    for i in range(n):
        for j in range(i + 1, n):
            if a[j] < a[i]:
                a[j], a[i] = a[i], a[j]
        # print(a)
    return a


x = [5, 2, 6, 7, 2, 1, 0, 3]
print(selectionSort(x))
