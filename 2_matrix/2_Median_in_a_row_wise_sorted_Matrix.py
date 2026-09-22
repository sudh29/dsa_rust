class Solution:
    def median(self, matrix, r, c):
        # code here
        #     	temp=[]
        #     	for i in matrix:
        #     	    temp.extend(i)
        #     	temp.sort()
        #     	n=r*c
        #     	idx=(n+1)//2
        #         return temp[idx-1]

        start = 0
        end = 2000
        n = r * c
        while start <= end:
            mid = (end + start) // 2
            ans = 0
            for i in range(0, r, 1):
                l = 0
                h = c - 1
                while l <= h:
                    m = l + (h - l) // 2
                    if matrix[i][m] <= mid:
                        l = m + 1
                    else:
                        h = m - 1
                ans += l
            if ans <= n / 2:
                start = mid + 1
            else:
                end = mid - 1
        return start
