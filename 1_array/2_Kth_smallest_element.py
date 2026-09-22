class Solution:
    def kthSmallest(self, arr, l, r, k):
        """
        arr : given array
        l : starting index of the array i.e 0
        r : ending index of the array i.e size-1
        k : find kth smallest element and return using this function
        """
        arr.sort()
        return arr[k - 1] if len(arr) >= k else -1


# Quickselect sort
import random


def quickselect(arr, k):
    left = 0
    right = len(arr) - 1
    while left <= right:
        pivot_idx = random.randint(left, right)
        new_pivot_idx = partition(arr, left, right, pivot_idx)
        if k == new_pivot_idx:
            return arr[k]
        elif k < new_pivot_idx:
            right = new_pivot_idx - 1
        else:
            left = new_pivot_idx + 1


def partition(arr, left, right, pivot_idx):
    pivot_val = arr[pivot_idx]
    arr[pivot_idx], arr[right] = arr[right], arr[pivot_idx]
    new_pivot_idx = left
    for i in range(left, right):
        if arr[i] < pivot_val:
            arr[i], arr[new_pivot_idx] = arr[new_pivot_idx], arr[i]
            new_pivot_idx += 1
    arr[right], arr[new_pivot_idx] = arr[new_pivot_idx], arr[right]
    return new_pivot_idx
