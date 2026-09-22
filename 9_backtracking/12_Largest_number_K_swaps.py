# User function Template for python3
def get_max(arr, max_val, idx, n):
    for i in range(idx + 1, n):
        if int(arr[i]) > int(max_val):
            max_val = arr[i]
    return max_val


def solve(arr, k, ans, idx, n):
    if idx == n or k == 0:
        return

    max_val = arr[idx]
    max_val = get_max(arr, max_val, idx, n)

    if arr[idx] != max_val:
        k -= 1

    for j in range(n - 1, idx - 1, -1):
        if arr[j] == max_val:
            arr[idx], arr[j] = arr[j], arr[idx]
            arr_str = "".join(arr)
            if int(arr_str) > int(ans[0]):
                ans[0] = arr_str
            solve(arr, k, ans, idx + 1, n)
            # backtrack
            arr[idx], arr[j] = arr[j], arr[idx]


class Solution:
    # Function to find the largest number after k swaps.
    def findMaximumNum(self, s, k):
        ans = [s]
        s = [i for i in s]
        solve(s, k, ans, 0, len(s))
        return ans[0]


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    for _ in range(int(input())):
        k = int(input())
        s = input()
        ob = Solution()
        print(ob.findMaximumNum(s, k))

# } Driver Code Ends
