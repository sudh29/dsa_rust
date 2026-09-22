# bubble sort


def bubbleSort(A):
    n = len(A)
    for k in range(1, n):
        flag = 0
        for i in range(n - k):
            if A[i] > A[i + 1]:
                A[i], A[i + 1] = A[i + 1], A[i]
                flag = 1
        if flag == 0:
            break
    return A


A = [8, 2, 6, 7, 2, 1, 0, 3]
print(bubbleSort(A))
