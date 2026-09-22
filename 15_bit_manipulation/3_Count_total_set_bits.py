def cal_x(n):
    x=0
    while ((1<< x) <= n):
        x+=1
    return x-1

def cal_total_bits(n):
    if n<=1:
        return n
    x=cal_x(n)
    msb= x * (1<<(x-1))
    n = n-(1<<x)
    res =  msb + n+1 + cal_total_bits(n)
    return res

class Solution:
    #Function to return sum of count of set bits in the integers from 1 to n.
    def countSetBits(self,n):
        # count = 0
        # for i in range(1, n + 1):
        #     count += bin(i).count('1')
        # return count
        
        
        # return cal_total_bits(n)
        
        count = 0
        i = 0
        while (1 << i) <= n:
            total_pairs = (n + 1) // (1 << (i + 1))
            remainder = (n + 1) % (1 << (i + 1))
            count += total_pairs * (1 << i) + max(0, remainder - (1 << i))
            i += 1
        return count
     
