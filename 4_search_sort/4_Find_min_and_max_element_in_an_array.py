def getMinMax(a: list, n: int) -> tuple:
    # 2n-2 comparisions
    min_val = float("inf")  # Initialize to infinity
    max_val = float("-inf")  # Initialize to negative infinity

    for i in range(n):
        if a[i] > max_val:
            max_val = a[i]
        if a[i] < min_val:
            min_val = a[i]

    return (min_val, max_val)

    # 3/2n-1/2 comparisions
    # if n == 0:
    #     return None, None

    # if n % 2 == 0:
    #     if a[0] < a[1]:
    #         mn, mx = a[0], a[1]
    #     else:
    #         mn, mx = a[1], a[0]
    #     i = 2
    # else:
    #     mn = mx = a[0]
    #     i = 1

    # while i < n - 1:
    #     if a[i] < a[i + 1]:
    #         if a[i] < mn:
    #             mn = a[i]
    #         if a[i + 1] > mx:
    #             mx = a[i + 1]
    #     else:
    #         if a[i + 1] < mn:
    #             mn = a[i + 1]
    #         if a[i] > mx:
    #             mx = a[i]
    #     i += 2
    # return mn, mx

# Example usage:
# arr = [1, 2, 3, 4, 5]
# print(getMinMax(arr, len(arr)))  # Output: (1, 5)
