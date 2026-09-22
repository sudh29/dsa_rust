#User function Template for python3
class Solution:
	def minJumps(self, arr, n):
	    if n == 1:
            return 0
        if arr[0] == 0:
            return -1
        jumps = 0
        current_end = 0
        farthest = 0
        for i in range(n - 1):
            farthest = max(farthest, i + arr[i])
            if i == current_end:
                jumps += 1
                current_end = farthest
                if current_end >= n - 1:
                    return jumps
        return -1

        # # DP
        # dp = [float('inf')] * n
        # dp[0] = 0
        # for i in range(1, n):
        #     for j in range(i):
        #         if i <= j + arr[j]:
        #             dp[i] = min(dp[i], dp[j] + 1)
        # return dp[-1] if dp[-1] != float('inf') else -1

#{
 # Driver Code Starts
#Initial Template for Python 3
if __name__ == '__main__':
	T=int(input())
	for i in range(T):
		n = int(input())
		Arr = [int(x) for x in input().split()]
		ob = Solution()
		ans = ob.minJumps(Arr,n)
		print(ans)
# } Driver Code Ends
