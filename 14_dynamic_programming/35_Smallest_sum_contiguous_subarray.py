# User function Template for python3


class Solution:
    def smallestSumSubarray(self, arr, n):
        # min_so_far = arr[0]
        # current_min = arr[0]
        # for i in range(1, n):
        #     current_min = min(arr[i], current_min + arr[i])
        #     min_so_far = min(min_so_far, current_min)
        # return min_so_far

        dp = [0] * n
        dp[0] = arr[0]
        min_sum = dp[0]
        for i in range(1, n):
            dp[i] = min(arr[i], dp[i - 1] + arr[i])
            min_sum = min(min_sum, dp[i])
        return min_sum


# {
# Driver Code Starts
# Initial Template for Python 3


def main():
    T = int(input())
    while T > 0:
        N = int(input())

        A = [int(x) for x in input().strip().split()]

        obj = Solution()
        print(obj.smallestSumSubarray(A, N))

        T -= 1


if __name__ == "__main__":
    main()
# } Driver Code Ends
