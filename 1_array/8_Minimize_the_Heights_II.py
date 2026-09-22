# User function Template for python3


class Solution:
    # Minimize the Heights I
    def getMinDiff_1(self, arr, n, k):
        if n == 1:
            return 0
        arr.sort()
        diff = arr[n - 1] - arr[0]
        for i in range(1, n):
            max_val = max(arr[i - 1] + k, arr[n - 1] - k)
            min_val = min(arr[0] + k, arr[i] - k)
            diff = min(diff, max_val - min_val)
        return diff

    # Minimize the Heights II
    def getMinDiff(self, arr, n, k):
        if n == 1:
            return 0
        arr.sort()
        diff = arr[n - 1] - arr[0]
        for i in range(1, n):
            if arr[i] - k < 0:
                continue
            max_val = max(arr[i - 1] + k, arr[n - 1] - k)
            min_val = min(arr[0] + k, arr[i] - k)
            diff = min(diff, max_val - min_val)
        return diff


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    tc = int(input())
    while tc > 0:
        k = int(input())
        n = int(input())
        arr = list(map(int, input().strip().split()))
        ob = Solution()
        ans = ob.getMinDiff(arr, n, k)
        print(ans)
        tc -= 1

# } Driver Code Ends
