class Solution:
    # Function to find the maximum number of meetings that can
    # be performed in a meeting room.
    def maximumMeetings(self, n, start, end):
        data = [[start[i], end[i]] for i in range(n)]
        data = sorted(data, key=lambda x: x[1])
        res = 1
        temp = data[0]
        for i in range(1, n):
            if data[i][0] > temp[1]:
                res += 1
                temp = data[i]
        return res


# {
# Driver Code Starts
# Initial Template for Python 3

# Contributed by : Nagendra Jha

if __name__ == "__main__":
    test_cases = int(input())
    for cases in range(test_cases):
        n = int(input())
        start = list(map(int, input().strip().split()))
        end = list(map(int, input().strip().split()))
        ob = Solution()
        print(ob.maximumMeetings(n, start, end))
# } Driver Code Ends
