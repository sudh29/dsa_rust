class Solution:
    def minJumps(self, arr, n):
        # code here
        if n == 1:
            return 0
        if arr[0] == 0:
            return -1
        halt = 0
        max_val = -100000
        jump = 0

        for i in range(n - 1):
            max_val = max(max_val, i + arr[i])
            if max_val >= n - 1:
                jump += 1
                return jump
            if i == halt:
                halt = max_val
                jump += 1
        if halt >= n - 1:
            return jump
        return -1
