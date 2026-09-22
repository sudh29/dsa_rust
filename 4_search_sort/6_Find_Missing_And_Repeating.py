def findTwoElement(arr: list, n: int) -> tuple:
    temp = [0, 0]  # Array to hold the two elements
    sum1 = sum(range(1, n + 1))  # Sum of first n natural numbers
    sum2 = sum(arr)  # Sum of the elements in the array

    for i in range(n):
        idx = abs(arr[i]) - 1
        if arr[idx] >= 0:
            arr[idx] = -arr[idx]  # Mark as visited by negating
        else:
            temp[0] = abs(arr[i])  # Duplicate element
            break

    sum2 -= temp[0]  # Remove duplicate from sum2
    temp[1] = sum1 - sum2  # The missing element

    return tuple(temp)  # Return as a tuple


# Example usage:
# arr = [3, 1, 3]
# n = len(arr)
# print(findTwoElement(arr, n))  # Output: (3, 2)
