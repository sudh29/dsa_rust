# User function Template for python3
import heapq


class Solution:
    # Function to find maximum of each subarray of size k.
    def max_of_subarrays(self, arr, n, k):
        ans = []
        heap = []
        # Initialize the heap with the first k elements
        for i in range(k):
            heapq.heappush(heap, (-arr[i], i))
        # The maximum element in the first window
        ans.append(-heap[0][0])
        # Process the remaining elements
        for i in range(k, len(arr)):
            heapq.heappush(heap, (-arr[i], i))
            # Remove elements that are outside the current window
            while heap[0][1] <= i - k:
                heapq.heappop(heap)
            # The maximum element in the current window
            ans.append(-heap[0][0])
        return ans


# {
# Driver Code Starts
# Initial Template for Python 3

import atexit
import io
import sys

# Contributed by : Nagendra Jha

_INPUT_LINES = sys.stdin.read().splitlines()
input = iter(_INPUT_LINES).__next__
_OUTPUT_BUFFER = io.StringIO()
sys.stdout = _OUTPUT_BUFFER


@atexit.register
def write():
    sys.__stdout__.write(_OUTPUT_BUFFER.getvalue())


if __name__ == "__main__":
    test_cases = int(input())
    for cases in range(test_cases):
        n, k = map(int, input().strip().split())
        arr = list(map(int, input().strip().split()))
        ob = Solution()
        res = ob.max_of_subarrays(arr, n, k)
        for i in range(len(res)):
            print(res[i], end=" ")
        print()
# } Driver Code Ends
