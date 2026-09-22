def mergesort(arr, s, e):
    count = 0
    if s < e:
        mid = (e + s) // 2
        count += mergesort(arr, s, mid)
        count += mergesort(arr, mid + 1, e)
        count += merge(arr, s, mid, e)
    return count


def merge(arr, s, m, e):
    c = 0
    n1 = m - s + 1
    n2 = e - m
    left = [0 for i in range(n1)]
    right = [0 for i in range(n2)]
    for i in range(n1):
        left[i] = arr[s + i]
    for j in range(n2):
        right[j] = arr[m + 1 + j]
    i = 0
    j = 0
    k = s
    while i < n1 and j < n2:
        if left[i] <= right[j]:
            arr[k] = left[i]
            i += 1
        else:
            arr[k] = right[j]
            c += n1 - i
            j += 1
        k += 1
    while i < n1:
        arr[k] = left[i]
        i += 1
        k += 1
    while j < n2:
        arr[k] = right[j]
        j += 1
        k += 1
    return c


class Solution:
    def inversionCount(self, arr, n):
        # op=0
        # for i in range(n):
        #     for j in range(i+1,n):
        #         if a[i]>a[j]:
        #             op+=1
        # return op

        return mergesort(arr, 0, n - 1)
