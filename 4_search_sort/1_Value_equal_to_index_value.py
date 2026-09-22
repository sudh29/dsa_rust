class Solution:
    def valueEqualToIndex(self, arr, n):
        res = []
        for i in range(n):
            if i + 1 == arr[i]:
                res.append(arr[i])
        return res
