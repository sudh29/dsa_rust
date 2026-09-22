def kadane(arr):
    max_ending_here = arr[0]
    max_so_far = arr[0]
    for i in range(1, len(arr)):
        max_ending_here = max(arr[i], max_ending_here + arr[i])
        max_so_far = max(max_so_far, max_ending_here)
    return max_so_far


class Solution:
    def maximumSumRectangle(self, R, C, M):
        max_sum = float("-inf")
        temp = [0] * C
        for top in range(R):
            temp = [0] * C
            for bottom in range(top, R):
                for i in range(C):
                    temp[i] += M[bottom][i]
                max_sum_subarray = kadane(temp)
                max_sum = max(max_sum, max_sum_subarray)
        return max_sum


# {
# Driver Code Starts
# Initial Template for Python 3

import sys

if __name__ == "__main__":
    t = int(sys.stdin.readline().strip())
    for _ in range(t):
        N, M = map(int, sys.stdin.readline().strip().split())
        a = []
        for i in range(N):
            s = list(map(int, sys.stdin.readline().strip().split()))
            a.append(s)
        ob = Solution()
        print(ob.maximumSumRectangle(N, M, a))
# } Driver Code Ends
