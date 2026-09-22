# User function Template for python3
MAX = 256


class Solution:
    def chooseandswap(self, A):
        A = [i for i in A]
        n = len(A)
        i, j = 0, 0
        chk = [-1 for i in range(MAX)]
        for i in range(n):
            if chk[ord(A[i])] == -1:
                chk[ord(A[i])] = i
        for i in range(n):
            flag = False
            for j in range(ord(A[i])):
                if chk[j] > chk[ord(A[i])]:
                    flag = True
                    break
            if flag:
                break
        if i < n - 1:
            ch1 = A[i]
            ch2 = chr(j)
            for i in range(n):
                if A[i] == ch1:
                    A[i] = ch2
                elif A[i] == ch2:
                    A[i] = ch1
        return "".join(A)


# {
# Driver Code Starts
# Initial Template for Python 3


if __name__ == "__main__":
    ob = Solution()
    t = int(input())
    for _ in range(t):
        A = input()
        ans = ob.chooseandswap(A)
        print(ans)


# } Driver Code Ends
