def find_pair(arr: list, size: int, n: int) -> bool:
    arr.sort()  # Sort the array
    start, end = 0, 1

    while start < size and end < size:
        diff = arr[end] - arr[start]
        if diff == n:
            return True
        elif diff < n:
            end += 1
        else:
            start += 1
            end = start + 1

    return False


# Example usage:
# arr = [1, 5, 3, 4, 2]
# n = 3
# print(find_pair(arr, len(arr), n))  # Output: True (because 5 - 2 = 3)
