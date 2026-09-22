def max_heapify(arr, N, i):
    largest = i
    l = 2 * i + 1
    r = 2 * i + 2
    if l < N and arr[l] > arr[largest]:
        largest = l
    if r < N and arr[r] > arr[largest]:
        largest = r
    if largest != i:
        arr[i], arr[largest] = arr[largest], arr[i]
        max_heapify(arr, N, largest)


def min_heapify(arr, N, i):
    smallest = i
    l = 2 * i + 1
    r = 2 * i + 2
    if l < N and arr[l] < arr[smallest]:
        smallest = l
    if r < N and arr[r] < arr[smallest]:
        smallest = r
    if smallest != i:
        arr[i], arr[smallest] = arr[smallest], arr[i]
        min_heapify(arr, N, smallest)


def max_buildHeap(arr, N):
    idx = N // 2 - 1
    for i in range(idx, -1, -1):
        max_heapify(arr, N, i)


def min_buildHeap(arr, N):
    idx = N // 2 - 1
    for i in range(idx, -1, -1):
        min_heapify(arr, N, i)


def printHeap(arr, N):
    print("Array representation of Heap is:")
    for i in range(N):
        print(arr[i], end=" ")
    print()


def insert_heap(arr, N, value):
    N = N + 1
    arr[N] = value
    i = N
    while i > 1:
        parent = i // 2
        if arr[parent] < arr[i]:
            arr[i], arr[parent] = arr[parent], arr[i]
            i = parent
        else:
            return


def delete_heap(arr, N):
    arr[1] = arr[N]
    N = N - 1
    i = 1
    while i < N:
        left = arr[i * 2]
        right = arr[i * 2 + 1]

        largest = i * 2 if left > right else i * 2 + 1
    if arr[i] < arr[largest]:
        arr[i], arr[largest] = arr[largest], arr[i]
        i = largest
    else:
        return


if __name__ == "__main__":
    arr = [1, 3, 5, 4, 6, 13, 10, 9, 8, 15, 17]
    N = len(arr)
    max_buildHeap(arr, N)
    printHeap(arr, N)

    arr = [1, 3, 5, 4, 6, 13, 10, 9, 8, 15, 17]
    N = len(arr)
    min_buildHeap(arr, N)
    printHeap(arr, N)
