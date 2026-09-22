# User function Template for python3


class Solution:
    def minSubset(self, A, N):
        A.sort()
        i = 1
        A1 = sum(A[N - i : N])
        A2 = sum(A[0 : N - i])
        if A1 > A2:
            return i
        j = N - i
        while j > 0:
            j -= 1
            i += 1
            A1 += A[j]
            A2 -= A[j]
            if A1 > A2:
                return i
        return i


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        N = int(input())
        A = list(map(int, input().strip().split()))
        ob = Solution()
        ans = ob.minSubset(A, N)
        print(ans)
# } Driver Code Ends
