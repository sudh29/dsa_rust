class Solution:
    def rearrange(self, arr, n):
        # Extra memory
        # arr1 = [i for i in arr if i < 0]
        # arr2 = [j for j in arr if j >= 0]
        # i = 0
        # while arr1 and arr2:
        #     arr[i] = arr2.pop(0)
        #     i += 1
        #     arr[i] = arr1.pop(0)
        #     i += 1
        # while arr1:
        #     arr[i] = arr1.pop(0)
        #     i += 1

        # while arr2:
        #     arr[i] = arr2.pop(0)
        #     i += 1

        # In place 
        n = len(arr)
        # Step 1: Partition (move negatives to left, positives to right)
        i, j = 0, n - 1
        while i <= j:
            if arr[i] < 0:
                i += 1
            elif arr[j] >= 0:
                j -= 1
            else:
                arr[i], arr[j] = arr[j], arr[i]
                i += 1
                j -= 1
        # At this point, all negatives are from 0..i-1
        neg, pos = 0, i
        # Step 2: Interleave negatives and positives
        while neg < pos < n and arr[neg] < 0:
            arr[neg], arr[pos] = arr[pos], arr[neg]
            neg += 2
            pos += 1
        return arr
