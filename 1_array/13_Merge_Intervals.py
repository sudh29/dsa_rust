class Solution:
    def merge(self, intervals: List[List[int]]) -> List[List[int]]:
        if len(intervals) == 1:
            return intervals
        intervals.sort()
        # print(intervals)
        temp = intervals[0]
        result = []
        flag = 0
        for i in range(1, len(intervals)):
            # print(temp,intervals[i])
            if temp[1] >= intervals[i][0]:
                temp = [min(temp[0], intervals[i][0]), max(temp[1], intervals[i][1])]
            else:
                result.append(temp)
                temp = intervals[i]
        result.append(temp)
        return result
