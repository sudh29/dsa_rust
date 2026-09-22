# User function Template for python3


class Solution:
    # def allPalindromicPerms(self, S):
    #     self.res = set()
    #     self.solve(list(S))
    #     return sorted(list(self.res))

    # def solve(self,arr):
    #     self.res.add(tuple(arr))
    #     if len(arr)<=1:
    #         return
    #     for i in range(1,len(arr)):
    #         if arr[i-1]==arr[i]:
    #             new_arr = arr[:i-1]+ [arr[i-1]+arr[i]] + arr[i+1:]
    #             self.solve(new_arr)
    #         if i+1< len(arr) and arr[i-1]==arr[i+1]:
    #             new_arr = arr[:i-1]+ [arr[i-1]+arr[i]+arr[i+1]] + arr[i+2:]
    #             self.solve(new_arr)

    def allPalindromicPerms(self, S):
        self.result = []
        self.backtrack(S, [])
        return self.result

    def backtrack(self, s, path):
        if not s:
            self.result.append(path[:])
            return

        for i in range(1, len(s) + 1):
            prefix = s[:i]
            if self.is_palindrome(prefix):
                self.backtrack(s[i:], path + [prefix])

    def is_palindrome(self, s):
        return s == s[::-1]


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        S = input()

        ob = Solution()
        allPart = ob.allPalindromicPerms(S)
        for i in range(len(allPart)):
            for j in range(len(allPart[i])):
                print(allPart[i][j], end=" ")
            print()
# } Driver Code Ends
