# User function Template for python3
"""
class Job:

    # Job class which stores profit and deadline.

    def __init__(self,profit=0,deadline=0):
        self.profit = profit
        self.deadline = deadline
        self.id = 0
"""


class Solution:
    def JobScheduling(self, jobs, n):
        jobs.sort(key=lambda x: x.profit, reverse=True)
        max_deadline = max(job.deadline for job in jobs)
        slot = [-1] * (max_deadline + 1)
        num_jobs = 0
        max_profit = 0
        for job in jobs:
            job_id = job.id
            deadline = job.deadline
            profit = job.profit
            for j in range(min(max_deadline, deadline), 0, -1):
                if slot[j] == -1:
                    slot[j] = job_id
                    num_jobs += 1
                    max_profit += profit
                    break
        return num_jobs, max_profit


# {
# Driver Code Starts
# Initial Template for Python 3


# Contributed by : Nagendra Jha
class Job:
    """
    Job class which stores profit and deadline.
    """

    def __init__(self, profit=0, deadline=0):
        self.profit = profit
        self.deadline = deadline
        self.id = 0


if __name__ == "__main__":
    test_cases = int(input())
    for cases in range(test_cases):
        n = int(input())
        info = list(map(int, input().strip().split()))
        Jobs = [Job() for i in range(n)]
        for i in range(n):
            Jobs[i].id = info[3 * i]
            Jobs[i].deadline = info[3 * i + 1]
            Jobs[i].profit = info[3 * i + 2]
        ob = Solution()
        res = ob.JobScheduling(Jobs, n)
        print(res[0], end=" ")
        print(res[1])

# } Driver Code Ends
