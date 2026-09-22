# User function Template for python3
# Manacher’s Algorithm
class Solution:
    def longestPalin(self, s):
        # Preprocess the string to add boundaries
        T = "^#" + "#".join(s) + "#$"
        n = len(T)
        P = [0] * n  # Array to store the radius of the palindrome at each index
        C, R = 0, 0  # Center and right boundary of the current palindrome

        # Main loop to calculate palindrome lengths
        for i in range(1, n - 1):
            mirror = 2 * C - i  # Mirror index of `i` with respect to `C`

            # Use the mirror property or reset radius
            if i < R:
                P[i] = min(R - i, P[mirror])

            # Expand the palindrome centered at `i`
            while T[i + P[i] + 1] == T[i - P[i] - 1]:
                P[i] += 1

            # Update the center and right boundary
            if i + P[i] > R:
                C, R = i, i + P[i]

        # Find the maximum palindrome length and its center
        max_len, center_index = max((P[i], i) for i in range(1, n - 1))

        # Extract the longest palindromic substring
        start = (
            center_index - max_len
        ) // 2  # Convert index in T back to original string
        return s[start : start + max_len]


def helper(s, left, right):
    while (left >= 0 and right < len(s)) and s[left] == s[right]:
        left -= 1
        right += 1
    return s[left + 1 : right]


class Solution:
    def longestPalin(self, s):
        # code here
        res = ""
        for i in range(len(s)):
            test = helper(s, i, i)
            if len(test) > len(res):
                res = test
            test = helper(s, i, i + 1)
            if len(test) > len(res):
                res = test
        return res if len(res) > 1 else s[0]


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())

    for _ in range(t):
        S = input()

        ob = Solution()
