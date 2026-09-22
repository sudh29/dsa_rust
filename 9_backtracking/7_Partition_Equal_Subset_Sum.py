# User function Template for Python3


def solve(arr, idx, curr_sum, target_sum):
    if curr_sum == target_sum:
        return True
    if curr_sum > target_sum or idx >= len(arr):
        return False
    if solve(arr, idx + 1, curr_sum + arr[idx], target_sum):
        return True
    if solve(arr, idx + 1, curr_sum, target_sum):
        return True
    return False


class Solution:
    def equalPartition(self, N, arr):
        total_sum = sum(arr)
        if total_sum % 2 != 0:
            return False
        target_sum = total_sum // 2
        return solve(arr, 0, 0, target_sum)


# {
# Driver Code Starts
# Initial Template for Python3

import sys

input = sys.stdin.readline
if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        N = int(input())
        arr = input().split()
        for it in range(N):
            arr[it] = int(arr[it])

        ob = Solution()
        if ob.equalPartition(N, arr) == 1:
            print("YES")
        else:
            print("NO")
# } Driver Code Ends
