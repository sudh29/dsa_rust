class Solution:
    def overlappedInterval(self, intervals):
        # Sort the intervals based on the starting times
        intervals.sort()
        res = []

        n = len(intervals)
        temp = intervals[0]  # Initialize with the first interval

        for i in range(n - 1):
            if intervals[i][1] >= intervals[i + 1][0]:  # Overlapping intervals
                # Merge the current interval with the next one
                intervals[i + 1][0] = intervals[i][0]
                intervals[i + 1][1] = max(intervals[i][1], intervals[i + 1][1])
                temp[0] = intervals[i + 1][0]
                temp[1] = intervals[i + 1][1]
            else:
                res.append(temp)  # Add the merged interval to result
                if i < n - 1:
                    temp = intervals[i + 1]  # Move to the next interval

        res.append(temp)  # Add the last merged interval to result
        return res


# Example usage:
solution = Solution()
intervals = [[1, 3], [2, 4], [5, 7], [6, 8]]
result = solution.overlappedInterval(intervals)
print(result)  # Output: [[1, 4], [5, 8]]
