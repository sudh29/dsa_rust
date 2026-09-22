import heapq


class Solution:
    # Function to merge k sorted arrays.
    def mergeKArrays(self, arr, k):
        min_heap = []
        for i in range(k):
            for j in range(k):
                heapq.heappush(min_heap, arr[i][j])

        res = [heapq.heappop(min_heap) for _ in range(k * k)]
        # print(res)
        return res


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        n = int(input())
        numbers = [[0 for _ in range(n)] for _ in range(n)]
        line = input().strip().split()
        for i in range(n):
            for j in range(n):
                numbers[i][j] = int(line[i * n + j])
        ob = Solution()
        merged_list = ob.mergeKArrays(numbers, n)
        for i in merged_list:
            print(i, end=" ")
        print()
# } Driver Code Ends
