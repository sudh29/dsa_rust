# User function Template for python3


def heapify(arr, n, i):
    largest = i
    left = 2 * i + 1
    right = 2 * i + 2

    if left < n and arr[left] > arr[largest]:
        largest = left
    if right < n and arr[right] > arr[largest]:
        largest = right
    if largest != i:
        arr[i], arr[largest] = arr[largest], arr[i]
        heapify(arr, n, largest)


def build_max_heap(arr):
    n = len(arr)
    for i in range(n // 2 - 1, -1, -1):
        heapify(arr, n, i)


class Solution:
    def mergeHeaps(self, a, b, n, m):
        # max_heap = []
        # i,j=0,0
        # while i<n and j<m:
        #     if a[i]>b[j]:
        #         heapq.heappush(max_heap,a[i])
        #         i+=1
        #     else:
        #         heapq.heappush(max_heap,b[j])
        #         j+=1
        # while i<n:
        #     heapq.heappush(max_heap,a[i])
        #     i+=1
        # while j<m:
        #     heapq.heappush(max_heap,b[j])
        #     j+=1
        # heapq._heapify_max(max_heap)
        # return max_heap

        merged_heap = a + b
        build_max_heap(merged_heap)
        return merged_heap


# {
# Driver Code Starts
# Initial Template for Python 3


def isMerged(arr1, arr2, merged):
    if len(arr1) + len(arr2) != len(merged):
        return False
    arr1 += arr2
    arr1.sort()
    mergedCopy = sorted(merged)
    if arr1 != mergedCopy:
        return False
    for i in range(1, len(merged)):
        if merged[i] > merged[(i - 1) // 2]:
            return False

    return True


if __name__ == "__main__":
    for _ in range(int(input())):
        n, m = map(int, input().split())
        a = [int(i) for i in input().split()]
        b = [int(i) for i in input().split()]
        copyA = a[:]
        copyB = b[:]
        obj = Solution()
        merged = obj.mergeHeaps(a, b, n, m)
        flag = isMerged(copyA, copyB, merged)
        print(0 if flag == False else 1)

# } Driver Code Ends
