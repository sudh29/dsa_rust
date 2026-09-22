# User function Template for python3
def max_heapify(arr, N, i):
    largest = i
    l = 2 * i + 1
    r = 2 * i + 2
    if l < N and arr[l] > arr[largest]:
        largest = l
    if r < N and arr[r] > arr[largest]:
        largest = r
    if largest != i:
        arr[i], arr[largest] = arr[largest], arr[i]
        max_heapify(arr, N, largest)


class Solution:
    def convertMinToMaxHeap(self, N, arr):
        idx = N // 2 - 1
        for i in range(idx, -1, -1):
            max_heapify(arr, N, i)


# {
# Driver Code Starts.
if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        N = int(input())
        arr = list(map(int, input().split()))
        ob = Solution()
        ob.convertMinToMaxHeap(N, arr)
        for val in arr:
            print(val, end=" ")
        print()
# } Driver Code Ends
