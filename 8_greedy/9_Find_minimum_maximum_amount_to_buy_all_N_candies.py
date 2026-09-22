# User function Template for python3


class Solution:
    def candyStore(self, candies, N, K):
        min_val = 0
        max_val = 0
        candies.sort()
        j = N - 1
        m = 0
        n = N - 1
        for i in range(N):
            if i <= j:
                min_val += candies[i]
                j -= K
            if m <= n:
                max_val += candies[n]
                m += K
                n -= 1

        return min_val, max_val


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())

    for _ in range(t):
        N, K = [int(x) for x in input().split()]
        candies = [int(x) for x in input().split()]

        solObj = Solution()

        print(*solObj.candyStore(candies, N, K))
