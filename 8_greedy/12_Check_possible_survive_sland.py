class Solution:
    def minimumDays(self, S, N, M):
        if M > N or (S > 6 and (N * 6) < (M * 7)):
            return -1
        total = S * M
        res = total // N
        if total % N > 0:
            res += 1
        return res


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        S, N, M = [int(x) for x in input().split()]

        ob = Solution()
        print(ob.minimumDays(S, N, M))
