# User function Template for python3


class Solution:
    def minFlips(self, S):
        n = len(S)
        flips_starting_with_0 = 0
        flips_starting_with_1 = 0
        for i in range(n):
            expected_char_0 = "0" if i % 2 == 0 else "1"
            expected_char_1 = "1" if i % 2 == 0 else "0"
            if S[i] != expected_char_0:
                flips_starting_with_0 += 1
            if S[i] != expected_char_1:
                flips_starting_with_1 += 1
        return min(flips_starting_with_0, flips_starting_with_1)


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())
    for i in range(t):
        S = input()
        Obj = Solution()
        ans = Obj.minFlips(S)
        print(ans)
# } Driver Code Ends
