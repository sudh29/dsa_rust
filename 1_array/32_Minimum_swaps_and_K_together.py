import sys


def minSwap(arr, n, k):
    # Complete the function
    res = sys.maxsize
    swaps = 0
    fav = 0
    no_fav = 0
    for i in range(n):
        if arr[i] <= k:
            fav += 1
    for j in range(fav):
        if arr[j] > k:
            no_fav += 1
    s = 0
    e = fav - 1
    while e < n:
        res = min(res, no_fav)
        e += 1
        if e < n and arr[e] > k:
            no_fav += 1
        if s < n and arr[s] > k:
            no_fav -= 1
        s += 1

    return 0 if res == sys.maxsize else res
