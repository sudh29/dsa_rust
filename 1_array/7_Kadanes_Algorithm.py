class Solution:
    ##Complete this function
    # Function to find the sum of contiguous subarray with maximum sum.
    def maxSubArraySum(self, arr, N):
        ##Your code here
        max_so_far = -100000000
        max_end_her = 0
        for i in arr:
            max_end_her = max(i, max_end_her + i)
            max_so_far = max(max_so_far, max_end_her)
        return max_so_far


def max_subarray_sum(arr):
    max_ending_here = max_so_far = arr[0]
    for i in range(1, len(arr)):
        max_ending_here = max(arr[i], max_ending_here + arr[i])
        max_so_far = max(max_so_far, max_ending_here)
    return max_so_far


arr = [-2, -3, 4, -1, -2, 1, 5, -3]
print(max_subarray_sum(arr))
