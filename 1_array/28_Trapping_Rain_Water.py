class Solution:
    def trappingWater(self, arr, n):
        # Code here
        # left=[arr[0]]
        # right=[arr[n-1]]
        # l_max=arr[0]
        # r_max=arr[n-1]
        # for i in range(1,n):
        #     l_max=max(l_max,arr[i])
        #     left.append(l_max)
        #     r_max=max(r_max,arr[n-1-i])
        #     right.append(r_max)
        # right=right[::-1]
        # res=0
        # for i in range(n):
        #     temp=min(left[i],right[i])
        #     res+=(temp-arr[i])
        res = 0
        i = 0
        j = n - 1
        left_max = arr[0]
        right_max = arr[n - 1]
        while i <= j:
            if arr[i] < arr[j]:
                if arr[i] < left_max:
                    res += left_max - arr[i]
                else:
                    left_max = arr[i]
                i += 1
            else:
                if arr[j] < right_max:
                    res += right_max - arr[j]
                else:
                    right_max = arr[j]
                j -= 1
        return res
