def kthSmallest(mat, n, k):
    # Your code goes here
    # temp=[]
    # for i in mat:
    #     temp.extend(i)
    # temp.sort()
    # return temp[k-1]

    low = mat[0][0]
    high = mat[n - 1][n - 1]

    while low <= high:
        mid = (high + low) // 2
        # print(low,high,mid)
        ans = 0
        for i in range(n):
            l = 0
            h = n - 1
            while l <= h:
                m = l + (h - l) // 2
                if mat[i][m] <= mid:
                    l = m + 1
                else:
                    h = m - 1
            ans += l
        if ans < k:
            low = mid + 1
        else:
            high = mid - 1
    return low
