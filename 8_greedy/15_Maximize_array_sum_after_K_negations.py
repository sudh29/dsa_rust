# User function Template for python3


class Solution:
    def maximizeSum(self, a, n, k):
        a.sort()
        i = 0
        for i in range(n):
            if k and a[i] < 0:
                a[i] *= -1
                k -= 1
                continue
            break
        if k == 0 or k % 2 == 0:
            return sum(a)

        if i == n:
            i -= 1
        if i != 0 and abs(a[i]) >= abs(a[i - 1]):
            i -= 1
        a[i] *= -1
        return sum(a)


# {
# Driver Code Starts
# Initial Template for Python 3


def main():
    T = int(input())

    while T > 0:
        sz = [int(x) for x in input().strip().split()]
        n, k = sz[0], sz[1]
        a = [int(x) for x in input().strip().split()]
        ob = Solution()
        print(ob.maximizeSum(a, n, k))

        T -= 1


if __name__ == "__main__":
    main()
