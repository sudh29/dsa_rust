from typing import List
import heapq


class Solution:
    def kthLargest(self, N: int, K: int, arr: List[int]) -> int:
        sum_val = [0] * (N + 1)
        for i in range(1, N + 1):
            sum_val[i] = sum_val[i - 1] + arr[i - 1]
        # print(sum_val)
        min_heap = []
        heapq.heapify(min_heap)
        for i in range(1, N + 1):
            for j in range(i, N + 1):
                x = sum_val[j] - sum_val[i - 1]
                if len(min_heap) < K:
                    heapq.heappush(min_heap, x)
                else:
                    if min_heap[0] < x:
                        heapq.heappop(min_heap)
                        heapq.heappush(min_heap, x)
        # print(min_heap)
        return min_heap[0]


# {
# Driver Code Starts
class IntArray:
    def __init__(self) -> None:
        pass

    def Input(self, n):
        arr = [int(i) for i in input().strip().split()]  # array input
        return arr

    def Print(self, arr):
        for i in arr:
            print(i, end=" ")
        print()


if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        N = int(input())

        K = int(input())

        Arr = IntArray().Input(N)

        obj = Solution()
        res = obj.kthLargest(N, K, Arr)

        print(res)


# } Driver Code Ends
