def search(arr: list, n: int, x: int, k: int) -> int:
    i = 0
    while i < n:
        if arr[i] == x:
            return i
        # Calculate the next index to check
        i += max(1, (arr[i] - k) // x)
    return -1


# Example usage:
# arr = [10, 20, 30, 40, 50]
# x = 30
# k = 5
# print(search(arr, len(arr), x, k))  # Output: 2
