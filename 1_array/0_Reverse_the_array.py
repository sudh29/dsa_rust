class Solution:
    def reverseWord(self, str: str) -> str:
        n = len(str)
        str = list(str)
        for i in range(n // 2):
            str[i], str[n - i - 1] = str[n - i - 1], str[i]

        # Two pointer approach
        # start = 0
        # end = n - 1
        # while start < end:
        #     str[start], str[end] = str[end], str[start]
        #     start += 1
        #     end -= 1
        str = "".join(str)
        return str


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())
    while t > 0:
        s = input()
        ob = Solution()
        print(ob.reverseWord(s))
        t = t - 1

# } Driver Code Ends
