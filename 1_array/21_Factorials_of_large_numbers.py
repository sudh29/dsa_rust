class Solution:
    def factorial(self, N):
        # code here
        temp = 1
        while N > 0:
            temp *= N
            N -= 1
        temp = [int(i) for i in str(temp)]
        # print(temp)
        return temp
