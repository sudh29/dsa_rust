def findMinInsertionsDP(str1, n):
    table = [[0 for i in range(n)] for i in range(n)]
    l, h, gap = 0, 0, 0
    for gap in range(1, n):
        l = 0
        for h in range(gap, n):
            if str1[l] == str1[h]:
                table[l][h] = table[l + 1][h - 1]
            else:
                table[l][h] = min(table[l][h - 1], table[l + 1][h]) + 1
            l += 1
    return table[0][n - 1]


class Solution:
    def countMin(self, Str):
        return findMinInsertionsDP(Str, len(Str))


def merge(ar):
    # Write your code here
    n = len(ar)
    s = 0
    e = n - 1
    res = 0
    while s <= e:
        if ar[s] == ar[e]:
            s += 1
            e -= 1
        elif ar[s] < ar[e]:
            s += 1
            ar[s] = ar[s] + ar[s - 1]
            res += 1
        else:
            e -= 1
            ar[e] = ar[e] = ar[e + 1]
            res += 1
    return res
