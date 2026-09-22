# User function Template for python3


class Solution:
    def countSubArrayProductLessThanK(self, a, n, k):
        start = 0
        prod = 1
        count = 0
        for end in range(n):
            prod *= a[end]
            while start <= end and prod >= k:
                prod //= a[start]
                start += 1
            count += end - start + 1
        return count

        # # Non contiguous
        # if k <= 1:
        #     return 0
        # dp = [0] * (k + 1)
        # result = 0
        # for j in range(1, n + 1):
        #     for i in range(k, 0, -1):
        #         if a[j - 1] <= i and a[j - 1] > 0:
        #             dp[i] += dp[i // a[j - 1]] + 1
        # return dp[k]


# {
# Driver Code Starts

# Initial Template for Python 3


def main():
    T = int(input())

    while T > 0:
        n, k = [int(x) for x in input().strip().split()]
        arr = [int(x) for x in input().strip().split()]

        print(Solution().countSubArrayProductLessThanK(arr, n, k))

        T -= 1


if __name__ == "__main__":
    main()


# } Driver Code Ends
