# User function Template for python3
import heapq


class Solution:
    def solve(self, arr, n):
        if n == 0:
            return 0
        if n == 1:
            return arr[0]

        heapq.heapify(arr)
        num1 = 0
        num2 = 0
        while len(arr) > 1:
            num1 = num1 * 10 + heapq.heappop(arr)
            num2 = num2 * 10 + heapq.heappop(arr)
        if len(arr) == 1:
            num1 = num1 * 10 + heapq.heappop(arr)
        return num1 + num2


# {
# Driver Code Starts
# Initial Template for Python 3


if __name__ == "__main__":
    tc = int(input())
    while tc > 0:
        n = int(input())
        arr = list(map(int, input().strip().split()))
        ob = Solution()
        ans = ob.solve(arr, n)
        print(ans)
        tc -= 1

# } Driver Code Ends
