# User function Template for python3
class Solution:
    def rowWithMax1s(self, arr, n, m):
        # code here
        # 		max_count=-100000
        # 		for i in range(n):
        # 		    temp=sum(arr[i])
        # 		    if max_count<temp:
        # 		        max_count=temp
        # 		        idx=i
        # 		return idx if max_count!=0 else -1

        #         max_count=-100000
        #         for i in range(n):
        #             l=0
        #             h=m-1
        #             while(l<=h):
        #                 mid=l+(h-l)//2
        #                 if arr[i][mid]==1:
        #                     h=mid-1
        #                 else:
        #                     l=mid+1
        #             if l==0:
        #                 return i
        #             temp=m-l
        #             if max_count<temp:
        # 		        max_count=temp
        # 		        idx=i
        # 		return idx if max_count!=0 else -1
        row = -1
        col = m - 1
        for i in range(n):
            # print(i,row,col)
            for j in range(col, -1, -1):
                if arr[i][j] == 1:
                    col -= 1
                    row = i
                else:
                    break
            if col == -1:
                return row
        return row
