class Solution:
    def maxGold(self, n, m, M):
        for col in range(m - 2, -1, -1):
            for row in range(n):
                right = M[row][col + 1]
                right_up = M[row - 1][col + 1] if row > 0 else 0
                right_down = M[row + 1][col + 1] if row < n - 1 else 0
                M[row][col] += max(right, right_up, right_down)
        return max(M[row][0] for row in range(n))


# {
# Driver Code Starts
# Initial Template for Python3

if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        n, m = [int(x) for x in input().split()]
        tarr = [int(x) for x in input().split()]
        M = []
        j = 0
        for i in range(n):
            M.append(tarr[j : j + m])
            j = j + m

        ob = Solution()
        print(ob.maxGold(n, m, M))
# } Driver Code Ends
