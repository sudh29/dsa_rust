# User function Template for python3
def binomialCoefficient(n, k):
    return factorial(n) // (factorial(k) * factorial(n - k))


def factorial(n):
    if n == 0:
        return 1
    else:
        return n * factorial(n - 1)


def solve(m, n, path, i, j, res):
    if i == m - 1 and j == n - 1:
        res.append(path)
        return
    if i < 0 or i >= m or j < 0 or j >= n:
        return
    if j + 1 < n:
        solve(m, n, path + "R", i, j + 1, res)
    if i + 1 < m:
        solve(m, n, path + "D", i + 1, j, res)


class Solution:
    def numberOfPaths(self, m, n):
        # 		# find all paths
        # 		res=[]
        # 		solve(m,n,'',0,0,res)
        # 		return len(res)

        #         # Count number of paths
        #         if(m == 1 or n == 1):
        #             return 1
        #         return self.numberOfPaths(m-1, n) + self.numberOfPaths(m, n-1)

        num_paths = binomialCoefficient(m + n - 2, m - 1)
        return num_paths % (10**9 + 7)


# {
# Driver Code Starts
# Initial Template for Python 3
if __name__ == "__main__":
    T = int(input())
    for i in range(T):
        m, n = input().split()
        m = int(m)
        n = int(n)
        ob = Solution()
        print(ob.numberOfPaths(m, n))

# } Driver Code Ends
