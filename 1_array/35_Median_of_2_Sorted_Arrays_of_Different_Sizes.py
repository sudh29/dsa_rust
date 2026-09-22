class Solution:
    def MedianOfArrays(self, array1, array2):
            #code here
        '''v=array1+array2
        v.sort()
        #print(v)
        n=len(v)
        if n%2==0:
            x=v[n//2]+v[(n//2)-1]
		    return x//2 if x%2==0 else x/2
	    else:
	        return v[n//2]'''
	    m=len(array1)
	    n=len(array2)
	    l=0
	    h=m+n
	    i=0
	    j=0
	    temp=[0 for i in range(h)]
	    while l<h:
	        #print(l,h,i,j)
	        if i<m and j<n and array1[i]<=array2[j]:
	            temp[l]=array1[i]
	            i+=1
	        elif i<m and j<n and array1[i]>array2[j]:
	            temp[l]=array2[j]
	            j+=1
	        elif i<m and j>=n:
	            temp[l]=array1[i]
	            i+=1
	        elif i>=m and j<n:
	            temp[l]=array2[j]
	            j+=1
	        l+=1
	    #print(temp)
	    v=temp
	    n=h
	    if n%2==0:
            x=v[n//2]+v[(n//2)-1]
		    return x//2 if x%2==0 else x/2
	    else:
	        return v[n//2]
