def find(arr, n, x):
    start = 0
    end = n - 1  # Adjusted for zero-based indexing in Python
    temp = float("inf")
    res = []

    while start <= end:
        mid = (start + end) // 2
        if arr[mid] == x:
            temp = mid
            break
        elif arr[mid] > x:
            end = mid - 1
        else:
            start = mid + 1

    if temp == float("inf"):
        return [-1, -1]

    i = temp
    while i < n and arr[i] == x:  # Check to avoid index out of range
        res.append(i)
        i += 1

    i = temp - 1
    while i >= 0 and arr[i] == x:  # Check to avoid index out of range
        res.insert(0, i)
        i -= 1

    m = len(res)
    return [res[0], res[m - 1]]
