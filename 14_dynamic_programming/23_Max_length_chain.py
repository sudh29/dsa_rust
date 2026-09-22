# User function Template for python3

"""
class Pair(object):
    def __init__(self, a, b):
        self.a = a
        self.b = b
"""


class Solution:
    def maxChainLen(self, P, n):
        # Greedy
        P.sort(key=lambda x: x.b)
        max_length = 1
        last_selected_end = P[0].b
        for i in range(1, n):
            if P[i].a > last_selected_end:
                max_length += 1
                last_selected_end = P[i].b
        return max_length

        # # DP
        # P.sort(key=lambda x: x.a)
        # dp = [1] * n
        # for i in range(1, n):
        #     for j in range(i):
        #         if P[j].b < P[i].a:
        #             dp[i] = max(dp[i], dp[j] + 1)
        # return max(dp)


# {
# Driver Code Starts
# Initial Template for Python 3


class Pair(object):
    def __init__(self, a, b):
        self.a = a
        self.b = b


if __name__ == "__main__":
    tcs = int(input())

    for _ in range(tcs):
        n = int(input())

        arr = [int(x) for x in input().split()]

        Parr = []

        i = 0
        while n * 2 > i:
            Parr.append(Pair(arr[i], arr[i + 1]))

            i += 2

        # print(Parr,len(Parr))
        obj = Solution()
        print(obj.maxChainLen(Parr, n))
# } Driver Code Ends
