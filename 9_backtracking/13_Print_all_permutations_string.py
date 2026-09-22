def solve(S, index, res):
    if index == len(S) - 1:
        res.append("".join(S))
        return
    seen = set()
    for i in range(index, len(S)):
        if S[i] not in seen:
            seen.add(S[i])
            S[index], S[i] = S[i], S[index]
            solve(S, index + 1, res)
            S[index], S[i] = S[i], S[index]


class Solution:
    def find_permutation(self, S):
        S = sorted(S)
        res = []
        solve(S, 0, res)
        return res


# {
# Driver Code Starts
# Initial Template for Python 3


if __name__ == "__main__":
    t = int(input())
    for i in range(t):
        S = input()
        ob = Solution()
        ans = ob.find_permutation(S)
        ans.sort()
        for i in ans:
            print(i, end=" ")
        print()
# } Driver Code Ends
