class Solution:
    def find_median(self, v):
        v.sort()
        n = len(v)
        if n % 2 == 0:
            return (v[n // 2] + v[(n // 2) - 1]) // 2
        else:
            return v[n // 2]
