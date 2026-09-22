class Solution:
    def countBitsFlip(self, a, b):
        # a_bin= bin(a)[2:]
        # b_bin=bin(b)[2:]
        # n=abs(len(a_bin)-len(b_bin))
        # if len(a_bin)>len(b_bin):
        #     for i in range(n):
        #         b_bin = '0'+b_bin
        # else:
        #     for i in range(n):
        #         a_bin= '0' + a_bin
        # #print(a_bin,b_bin)
        # c=0
        # for i in range(len(a_bin)):
        #     if a_bin[i]!=b_bin[i]:
        #         c+=1
        # return c
        
        # xor = a ^ b
        # count = 0
        # while xor:
        #     count += xor & 1
        #     xor >>= 1
        # return count
        
        return bin(a ^ b).count('1')
