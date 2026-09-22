# User function Template for python3
class Solution:
    def minimumNumberOfSwaps(self, S):
        open_count, close_count, UB, swaps = 0, 0, 0, 0
        for char in S:
            if char == "[":
                open_count += 1
                if UB > 0:
                    swaps += UB
                    UB -= 1
            else:
                close_count += 1
                UB = close_count - open_count
        return swaps


# {
# Driver Code Starts
# Initial Template for Python 3
if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        S = str(input())
        ob = Solution()
        print(ob.minimumNumberOfSwaps(S))
# } Driver Code Ends
