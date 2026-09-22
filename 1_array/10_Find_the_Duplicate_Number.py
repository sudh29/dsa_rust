class Solution:
    def findDuplicate(self, nums: List[int]) -> int:
        """sum_nums=sum(nums)
        nums_set=list(set(nums))
        sum_nums_set=sum(nums_set)
        return sum_nums-sum_nums_set"""

        #         tem={}
        #         for i in nums:
        #             if i not in tem:
        #                 tem[i]=1
        #             else:
        #                 tem[i]+=1
        #         for k,v in tem.items():
        #             if v>1:
        #                 return k
        #         return

        n = len(nums)
        indx = 0
        for i in range(n):
            indx = abs(nums[i]) - 1
            if nums[indx] < 0:
                return abs(nums[i])
            nums[indx] = -nums[indx]
        return -1
