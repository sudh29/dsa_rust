class Solution:

	# Function to find maximum
	# product subarray
	def maxProduct(self,arr, n):
		# code here
# 		max_pro=arr[0]
# 		min_p=arr[0]
# 		res=arr[0]
# 		curr=0
# 		for i in range(1,n):
# 		    curr=arr[i]
# 		  #  print(max_pro,min_p,res,curr)
# 		    if curr==0:
# 		        max_pro=1
# 		        min_p=1
# 		    else:
# 		        temp1=max_pro*curr
# 		        temp2=min_p*curr
#     		    max_pro=max(max(temp1,temp2),curr)
#     		    min_p=min(min(temp1,temp2),curr)
# 		    res=max(res,max_pro)
# 		return res

	        minprod = arr[0]
		maxprod = arr[0]
		res = arr[0]
		for i in range(1, n):
		    min1=minprod*arr[i]
		    max1=maxprod*arr[i]
		    minprod= min(arr[i],min1,max1)
		    maxprod=max(arr[i],max1,min1)
		    res=max(res,maxprod)
		return res
