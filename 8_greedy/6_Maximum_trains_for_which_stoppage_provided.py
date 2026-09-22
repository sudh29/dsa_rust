class Solution:
    def maxStop(self, n, m, trains):
        trains = sorted(trains, key=lambda x: x[1])
        platform = [-1] * (n + 1)
        count = 0
        for i in range(m):
            train = trains[i]
            if platform[train[2]] == -1:
                count += 1
                platform[train[2]] = train
            else:
                if platform[train[2]][1] <= train[0]:
                    platform[train[2]] = train
                    count += 1
        return count


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    for _ in range(int(input())):
        n, m = map(int, input().split())
        trains = []
        for i in range(m):
            trains.append([int(i) for i in input().split()])
        print(Solution().maxStop(n, m, trains))
# } Driver Code Ends
