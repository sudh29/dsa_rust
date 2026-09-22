# User function Template for python3
import heapq


class Solution:
    def __init__(self):
        self.a = []  # Max heap
        self.b = []  # Min heap
        self.median = 0

    def balanceHeaps(self):
        temp = -heapq.heappop(self.a)
        heapq.heappush(self.b, temp)
        if len(self.b) > len(self.a):
            temp = heapq.heappop(self.b)
            heapq.heappush(self.a, -temp)

    def getMedian(self):
        if len(self.a) != len(self.b):
            self.median = -self.a[0]
        else:
            self.median = (-self.a[0] + self.b[0]) / 2
        return self.median

    def insertHeaps(self, x):
        heapq.heappush(self.a, -x)
        self.balanceHeaps()


# {
# Driver Code Starts.

if __name__ == "__main__":
    t = int(input())

    for _ in range(t):
        n = int(input())
        ob = Solution()
        for i in range(n):
            x = int(input())
            ob.insertHeaps(x)
            print(math.floor(ob.getMedian()))

# } Driver Code Ends
