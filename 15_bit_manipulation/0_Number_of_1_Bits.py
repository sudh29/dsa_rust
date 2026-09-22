def binary(n):
    res = ""
    while n >= 1:
        rem = n % 2
        res += str(int(rem))
        n = n / 2
    return res[::-1]


class Solution:
    def setBits(self, N):
        # code here
        bin_n = binary(N)
        c = 0
        for i in bin_n:
            if i == "1":
                c += 1
        return c
