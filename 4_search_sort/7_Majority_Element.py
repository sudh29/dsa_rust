def majorityElement(arr: list) -> int:
    size = len(arr)
    k = size // 2

    # Using Moore’s Voting Algorithm
    maj_idx = 0
    count = 1

    for i in range(size):
        if arr[maj_idx] == arr[i]:
            count += 1
        else:
            count -= 1

        if count == 0:
            maj_idx = i
            count = 1

    # Count occurrences of the majority element
    c = 0
    for i in range(size):
        if arr[i] == arr[maj_idx]:
            c += 1

    if c > k:
        return arr[maj_idx]

    return -1


# Example usage:
# arr = [3, 1, 3, 3, 2]
# print(majorityElement(arr))  # Output: 3
