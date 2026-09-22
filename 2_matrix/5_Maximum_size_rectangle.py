# User function Template for python3
def max_histogram_area(hist):
    stack = []
    max_area = 0
    index = 0
    while index < len(hist):
        if not stack or hist[index] >= hist[stack[-1]]:
            stack.append(index)
            index += 1
        else:
            top_of_stack = stack.pop()
            area = hist[top_of_stack] * ((index - stack[-1] - 1) if stack else index)
            max_area = max(max_area, area)
    while stack:
        top_of_stack = stack.pop()
        area = hist[top_of_stack] * ((index - stack[-1] - 1) if stack else index)
        max_area = max(max_area, area)
    return max_area


class Solution:
    def maxArea(self, M, n, m):
        max_area = 0
        hist = [0] * m
        for i in range(n):
            for j in range(m):
                if M[i][j] == 0:
                    hist[j] = 0
                else:
                    hist[j] += 1
            max_area = max(max_area, max_histogram_area(hist))
        return max_area


# {
# Driver Code Starts
# Initial Template for Python 3


# Driver Code
if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        R, C = map(int, input().strip().split())
        A = []
        for i in range(R):
            line = [int(x) for x in input().strip().split()]
            A.append(line)
        print(Solution().maxArea(A, R, C))

# This code is contributed
# by SHUBHAMSINGH10

# } Driver Code Ends
