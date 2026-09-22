#User function Template for python3

class Solution:

	def removals(self,arr, n, k):

		# Two pointer
		arr.sort()
        min_removals = float('inf')
        j = 0
        for i in range(n):
            while j < n and arr[j] - arr[i] <= k:
                j += 1
            min_removals = min(min_removals, n - (j - i))
        return min_removals

        # DP

#{
 # Driver Code Starts
#Initial Template for Python 3


if __name__ == '__main__':
    tc = int(input())
    while tc > 0:
        n, k = list(map(int, input().strip().split()))
        arr = list(map(int, input().strip().split()))
        ob = Solution()
        ans = ob.removals(arr, n, k)
        print(ans)
        tc -= 1
# } Driver Code Ends
