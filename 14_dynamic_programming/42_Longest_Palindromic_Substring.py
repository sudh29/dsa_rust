class Solution:
    def longestPalindrome(self, s: str) -> str:
        # n = len(s)
        # if n == 0:
        #     return ""
        # dp = [[False] * n for _ in range(n)]
        # start = 0
        # max_length = 1
        # for i in range(n):
        #     dp[i][i] = True
        # for i in range(n - 1):
        #     if s[i] == s[i + 1]:
        #         dp[i][i + 1] = True
        #         start = i
        #         max_length = 2
        # for length in range(3, n + 1):
        #     for i in range(n - length + 1):
        #         j = i + length - 1
        #         if s[i] == s[j] and dp[i + 1][j - 1]:
        #             dp[i][j] = True
        #             start = i
        #             max_length = length
        # return s[start:start + max_length]

        n = len(s)
        if n == 0:
            return ""
        start = 0
        max_length = 1
        prev = [False] * n
        curr = [False] * n
        for i in range(n):
            for j in range(i, -1, -1):
                if s[i] == s[j] and (i - j <= 2 or prev[j + 1]):
                    curr[j] = True
                    if i - j + 1 > max_length:
                        start = j
                        max_length = i - j + 1
                else:
                    curr[j] = False
            prev = curr[:]
        return s[start : start + max_length]
