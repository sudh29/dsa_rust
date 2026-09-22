# User function Template for Python3


class Solution:
    def equalPartition(self, N, arr):
        total_sum = sum(arr)
        if total_sum % 2 != 0:
            return 0
        target = total_sum // 2
        dp = [False] * (target + 1)
        dp[0] = True
        for num in arr:
            for j in range(target, num - 1, -1):
                if dp[j - num]:
                    dp[j] = True
        return 1 if dp[target] else 0


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
