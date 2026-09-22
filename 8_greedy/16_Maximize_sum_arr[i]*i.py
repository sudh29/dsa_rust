# User function Template for python3


class Solution:
    def Maximize(self, a, n):
        # Complete the function
        mod = 10**9 + 7
        a.sort()
        sum_total = 0
        for i in range(n):
            sum_total = (sum_total + a[i] * i) % mod
        return sum_total


# {
# Driver Code Starts
# Initial Template for Python 3


for _ in range(0, int(input())):
    n = int(input())
    arr = list(map(int, input().strip().split()))
    ob = Solution()
    print(ob.Maximize(arr, n))

# } Driver Code Ends
