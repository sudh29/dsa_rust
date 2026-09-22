class Solution:
    def majorityElement(self, nums: List[int]) -> List[int]:
        temp = set()
        k = len(nums) // 3
        dictemp = {}
        for i in nums:
            if i not in dictemp:
                dictemp[i] = 1
            else:
                dictemp[i] += 1
        for i in nums:
            if dictemp[i] > k:
                temp.add(i)
        return temp
