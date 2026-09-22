import sys


class Solution:
    def findMinDiff(self, A, N, M):
        # code here
        A.sort()
        min_res = sys.maxsize
        for i in range(N - M + 1):
            # print(A[i:i+M])
            temp = A[i + M - 1] - A[i]
            min_res = min(min_res, temp)
        return min_res
