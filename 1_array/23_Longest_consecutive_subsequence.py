class Solution:
    # arr[] : the input array
    # N : size of the array arr[]

    # Function to return length of longest subsequence of consecutive integers.

    def findLongestConseqSubseq(self, arr, N):
        # code here
        # arr.sort()
        # temp=0
        # max_v=0
        # for i in range(N-1):
        #     if arr[i]+1==arr[i+1]:
        #         temp+=1
        #     elif arr[i]==arr[i+1]:
        #         pass
        #     else:
        #         temp=0
        #     max_v=max(max_v,temp)
        # return max_v+1

        HS = set(arr)
        res = -1000000
        for i in range(n):
            if (arr[i] - 1) not in HS:
                val = arr[i] + 1
                while val in HS:
                    val += 1
                res = max(res, val - arr[i])
        return res
