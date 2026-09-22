class Solution:
    def setAllRangeBits(self, N, L, R):
        # code here
        mask = 0
        for i in range(L, R + 1):
            temp = 1 << (i - 1)
            mask = mask | temp
        return N | mask
