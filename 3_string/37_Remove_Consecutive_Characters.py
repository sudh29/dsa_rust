# User function Template for python3


def solve(S):
    if len(S) < 2:
        return S
    if S[0] != S[1]:
        return S[0] + solve(S[1:])
    return solve(S[1:])


class Solution:
    def removeConsecutiveCharacter(self, S):
        n = len(S)
        if n < 2:
            return S
        S = [i for i in S]
        j = 0
        for i in range(n):
            if S[j] != S[i]:
                j += 1
                S[j] = S[i]
        j += 1
        S = S[:j]
        return "".join(S)

        # return solve(S)


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    T = int(input())

    for tcs in range(T):
        s = input()
        ob = Solution()
        print(ob.removeConsecutiveCharacter(s))

# } Driver Code Ends
