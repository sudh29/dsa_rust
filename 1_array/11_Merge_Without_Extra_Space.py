class Solution:
    def merge(self, arr1, arr2, n, m):
        # code here
        i = 0
        j = 0
        k = n - 1
        while i <= k and j < m:
            if arr1[i] < arr2[j]:
                i += 1
            else:
                arr1[k], arr2[j] = arr2[j], arr1[k]
                j += 1
                k -= 1
        arr1.sort()
        arr2.sort()

        # nums = []
        # i,j=0,0
        # while i<m and j<n:
        #     if nums1[i]<=nums2[j]:
        #         nums.append(nums1[i])
        #         i+=1
        #     else:
        #         nums.append(nums2[j])
        #         j+=1
        # while i<m:
        #     nums.append(nums1[i])
        #     i+=1
        # while j<n:
        #     nums.append(nums2[j])
        #     j+=1
        # for i in range(len(nums)):
        #     nums1[i] = nums[i]
        # return nums1

        # In place without extra space
        # i,j,k=m-1,n-1,m+n-1
        # while i>=0 and j>=0:
        #     if nums1[i]>nums2[j]:
        #         nums1[k]=nums1[i]
        #         i-=1
        #     else:
        #         nums1[k]=nums2[j]
        #         j-=1
        #     k-=1
        # while j>=0:
        #     nums1[k]=nums2[j]
        #     j-=1
        #     k-=1
        # return nums1
