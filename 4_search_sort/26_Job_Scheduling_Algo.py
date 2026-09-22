class Solution:
    def jobScheduling(self, startTime, endTime, profit):
        n = len(startTime)
        jobs = []

        # Create a list of jobs with start time, end time, and profit
        for i in range(n):
            jobs.append((startTime[i], endTime[i], profit[i]))

        # Sort jobs based on their end time
        jobs.sort(key=lambda x: x[1])

        # Initialize dp array to store maximum profit
        dp = [0] * n
        dp[0] = jobs[0][2]  # The profit of the first job

        for i in range(1, n):
            # Include the profit of the current job
            inc = jobs[i][2]
            last = -1
            low, high = 0, i - 1

            # Binary search to find the last non-conflicting job
            while low <= high:
                mid = (low + high) // 2
                if jobs[mid][1] <= jobs[i][0]:
                    last = mid
                    low = mid + 1
                else:
                    high = mid - 1

            # If there is a non-conflicting job, add its profit
            if last != -1:
                inc += dp[last]

            # Exclude the current job
            exc = dp[i - 1]

            # Store the maximum profit at this job
            dp[i] = max(inc, exc)

        return dp[n - 1]


# Example usage:
# sol = Solution()
# startTime = [1, 2, 3, 3]
# endTime = [3, 4, 5, 6]
# profit = [50, 10, 40, 70]
# print(sol.jobScheduling(startTime, endTime, profit))  # Output: 120
