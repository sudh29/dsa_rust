
class Solution:
	def LongestRepeatingSubsequence(self, s):
		# Code here
		def lrs_recursive(s, i, j, memo):
            if i == 0 or j == 0:
                return 0
            if memo[i][j] != -1:
                return memo[i][j]
            if s[i - 1] == s[j - 1] and i != j:
                memo[i][j] = 1 + lrs_recursive(s, i - 1, j - 1, memo)
            else:
                memo[i][j] = max(lrs_recursive(s, i, j - 1, memo), lrs_recursive(s, i - 1, j, memo))
            return memo[i][j]

        n = len(s)
        memo = [[-1 for _ in range(n + 1)] for _ in range(n + 1)]
        return lrs_recursive(s, n, n, memo)


#{
 # Driver Code Starts
#Initial Template for Python 3

if __name__ == '__main__':
	T=int(input())
	for i in range(T):
		str = input()
		ob = Solution()
		ans = ob.LongestRepeatingSubsequence(str)
		print(ans)

# } Driver Code Ends
