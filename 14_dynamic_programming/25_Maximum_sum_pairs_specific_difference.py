class Solution:
    def maxSumPairWithDifferenceLessThanK(self, arr, N, K):
        # # Greedy
        # arr.sort()
        # max_sum = 0
        # i = N - 1
        # while i > 0:
        #     if arr[i] - arr[i - 1] < K:
        #         max_sum += arr[i] + arr[i - 1]
        #         i -= 2
        #     else:
        #         i -= 1
        # return max_sum

        # DP
        arr.sort()
        dp = [0] * N
        dp[0] = 0
        for i in range(1, N):
            if i > 0 and arr[i] - arr[i - 1] < K:
                if i >= 2:
                    dp[i] = max(dp[i - 1], dp[i - 2] + arr[i] + arr[i - 1])
                else:
                    dp[i] = max(dp[i - 1], arr[i] + arr[i - 1])
            else:
                dp[i] = dp[i - 1]
        return dp[-1]


# {
# Driver Code Starts
# Initial Template for Python 3


def main():
    T = int(input())

    while T > 0:
        N = int(input())
        arr = [int(x) for x in input().strip().split()]
        K = int(input())
        ob = Solution()
        print(ob.maxSumPairWithDifferenceLessThanK(arr, N, K))

        T -= 1


if __name__ == "__main__":
    main()


# } Driver Code Ends
