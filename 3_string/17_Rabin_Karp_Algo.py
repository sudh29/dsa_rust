# User function Template for python3


def hashfn(x):
    xdict = {
        "a": 1,
        "b": 2,
        "c": 3,
        "d": 4,
        "e": 5,
        "f": 6,
        "g": 7,
        "h": 8,
        "i": 9,
        "j": 10,
        "k": 11,
        "l": 12,
        "m": 13,
        "n": 14,
        "o": 15,
        "p": 16,
        "q": 17,
        "r": 18,
        "s": 19,
        "t": 20,
        "u": 21,
        "v": 22,
        "w": 23,
        "x": 24,
        "y": 25,
        "z": 26,
    }
    x = list(x)
    val = 0
    for i in range(len(x)):
        val = val * 27
        val += xdict[x[i]]
    return val


class Solution:
    def search(self, b, a):
        res = []
        if not a or not b or len(a) < len(b):
            return [-1]
        x = hashfn(b)
        n = len(b)
        for i in range(len(a) - len(b) + 1):
            if hashfn(a[i : i + n]) == x:
                if a[i : i + n] == b:
                    res.append(i + 1)
        return res


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        s = input().strip()
        patt = input().strip()
        ob = Solution()
        ans = ob.search(patt, s)
        for value in ans:
            print(value, end=" ")
        print()
# } Driver Code Ends
