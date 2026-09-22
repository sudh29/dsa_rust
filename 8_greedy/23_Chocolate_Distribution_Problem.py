# User function Template for python3


class Solution:
    def findMinDiff(self, A, N, M):
        A.sort()
        res = float("inf")
        for i in range(0, N - M + 1):
            res = min(res, A[i + M - 1] - A[i])
        return res


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())

    for _ in range(t):
        N = int(input())
        A = [int(x) for x in input().split()]
        M = int(input())

        solObj = Solution()

        print(solObj.findMinDiff(A, N, M))
# } Driver Code Ends
