# User function Template for python3

VOWELS = {"a", "e", "i", "o", "u"}


def valid_str(s):
    return len(s) >= 2 and s[0] in VOWELS and s[-1] not in VOWELS


def solve(string, ans, res):
    if not string:
        if valid_str(ans):
            res.add(ans)
        return
    solve(string[1:], ans + string[0], res)
    solve(string[1:], ans, res)


class Solution:
    def allPossibleSubsequences(ob, S):
        res = set()
        solve(S, "", res)
        res = sorted(res)
        return res


# {
# Driver Code Starts
# Initial Template for Python 3
if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        S = input()
        ans = set()
        ob = Solution()
        ans = ob.allPossibleSubsequences(S)
        if len(ans) == 0:
            print(-1, end="")
        else:
            for i in ans:
                print(i, end=" ")
        print()
# } Driver Code Ends
