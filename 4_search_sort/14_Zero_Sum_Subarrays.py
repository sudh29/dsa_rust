def findSubarray(arr, n):
    res = 0
    curr_sum = 0
    sum_map = {}

    for i in range(n):
        curr_sum += arr[i]

        # If the current sum is 0, we have found a subarray
        if curr_sum == 0:
            res += 1

        # If the current sum has been seen before, add the number of times it has been seen to the result
        if curr_sum in sum_map:
            res += len(sum_map[curr_sum])

        # Add the current sum to the map with the current index
        if curr_sum not in sum_map:
            sum_map[curr_sum] = []

        sum_map[curr_sum].append(i)

    return res
