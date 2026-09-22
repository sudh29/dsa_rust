class Solution:
    # Function to find the maximum profit and the number of jobs done.
    def JobScheduling(self, Jobs, n):
        # Jobs_list = []
        # for i in range(n):
        #     Jobs_list.append([Jobs[i].id,Jobs[i].deadline,Jobs[i].profit])
        sorted_jobs_end_time = sorted(Jobs, key=lambda x: x.profit, reverse=True)
        array = [0] * n
        job_count = 0
        max_profit = 0
        for i in range(n):
            endtime = sorted_jobs_end_time[i].deadline
            for j in range(endtime - 1, -1, -1):
                if array[j] == 0:
                    array[j] = 1
                    job_count += 1
                    max_profit += sorted_jobs_end_time[i].profit
                    break
        return [job_count, max_profit]


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
