def rotate(arr, n):
    m = len(arr)
    rem = n % (m + 1)

    """for i in range(rem-1):
        temp=arr.pop(0)
        arr.append(temp)"""
    for i in range(rem - 1, 0, -1):
        arr[i], arr[i - 1] = arr[i - 1], arr[i]


# Rotate array by one
def cyclic_rotate(arr):
    last = arr[-1]
    for i in range(len(arr) - 1, 0, -1):
        arr[i] = arr[i - 1]
    arr[0] = last


arr = [1, 2, 3, 4, 5]
cyclic_rotate(arr)
print(arr)
