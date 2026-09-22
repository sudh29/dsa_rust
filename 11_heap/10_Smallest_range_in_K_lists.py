# User function Template for python3
import heapq


class Solution:
    def smallestRange(self, KSortedArray, n, k):
        low = 0
        high = 0
        range_val = float("inf")
        max_val = float("-inf")
        min_val = float("inf")
        heap = []
        i = 0
        for j in range(k):
            heapq.heappush(heap, (KSortedArray[j][i], i, j, len(KSortedArray[j])))
            min_val = min(min_val, KSortedArray[j][i])
            max_val = max(max_val, KSortedArray[j][i])

        while len(heap) > 0:
            min_element, a, b, arr_len = heapq.heappop(heap)
            if range_val > max_val - min_element:
                min_val = min_element
                range_val = max_val - min_val
                low = min_val
                high = max_val
            if a + 1 < arr_len:
                a += 1
                heapq.heappush(heap, (KSortedArray[b][a], a, b, arr_len))
                max_val = max(max_val, KSortedArray[b][a])
            else:
                break
        return [low, high]


# {
# Driver Code Starts
# Initial Template for Python 3

t = int(input())
for _ in range(t):
    line = input().strip().split()
    n = int(line[0])
    k = int(line[1])
    numbers = []
    for i in range(k):
        numbers.append([int(x) for x in input().strip().split()])
    r = Solution().smallestRange(numbers, n, k)
    print(r[0], r[1])
# } Driver Code Ends
