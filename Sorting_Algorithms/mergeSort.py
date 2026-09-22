# merge sort algorithm


def mergeSort(A):
    n = len(A)
    if n > 1:
        mid = n // 2
        left = A[:mid]
        right = A[mid:]
        mergeSort(left)
        mergeSort(right)

        i = j = k = 0
        while i < len(left) and j < len(right):
            if left[i] < right[j]:
                A[k] = left[i]
                i += 1
            else:
                A[k] = right[j]
                j += 1
            k += 1
        while i < len(left):
            A[k] = left[i]
            i += 1
            k += 1
        while j < len(right):
            A[k] = right[j]
            j += 1
            k += 1


a = [5, 2, 6, 7, 2, 1, 0, 3]
print(a)
(mergeSort(a))
print(a)
