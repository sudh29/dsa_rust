class Solution:
	def singleNumber(self, nums):
	    sums=0
        for i in nums:
            sums = sums ^ (i)
        right_set_bit = (sums & -sums)
        x=0
        y=0
        for i in nums:
            if i & right_set_bit:
                x=x^i
            else:
                y=y^i

        if x<y:
            return [x,y]
        else:
            return [y,x]
