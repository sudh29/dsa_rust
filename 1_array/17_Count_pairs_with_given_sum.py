class Solution:
    def getPairsCount(self, arr, n, k):
        # code here
        res = 0
        temp = {}
        for i in range(n):
            new_k = k - arr[i]
            if new_k in temp:
                res += temp[new_k]
            if arr[i] in temp:
                temp[arr[i]] += 1
            else:
                temp[arr[i]] = 1
        return res
