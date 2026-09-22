import heapq


class Solution:
    def kthSmallest(self, arr, l, r, k):
        """
        arr : given array
        l : starting index of the array i.e 0
        r : ending index of the array i.e size-1
        k : find kth smallest element and return using this function
        """
        min_heap = []
        for i in range(l, r + 1):
            heapq.heappush(min_heap, arr[i])

        if k > len(min_heap):
            return -1
        else:
            while k > 0:
                res = heapq.heappop(min_heap)
                k -= 1
        return res

    # Function to return k largest elements from an array.
    def kLargest(self, arr, n, k):
        max_heap = []
        for i in range(n):
            heapq.heappush(max_heap, -arr[i])

        if k > len(max_heap):
            return -1
        else:
            res = [-heapq.heappop(max_heap) for _ in range(k)]
        return res
