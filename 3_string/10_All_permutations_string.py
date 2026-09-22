# User function Template for python3
def permute(s):
    if len(s) == 0:
        return [""]
    permutations = []
    for i in range(len(s)):
        char = s[i]
        remaining = s[:i] + s[i + 1 :]
        for p in permute(remaining):
            permutations.append(char + p)
    return permutations


class Solution:
    def find_permutation(self, s):
        p = permute(s)
        p = list(set(p))
        return p


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
