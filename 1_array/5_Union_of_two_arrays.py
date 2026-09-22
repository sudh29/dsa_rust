class Solution:
    # Function to return the count of number of elements in union of two arrays.
    def doUnion(self, a, n, b, m):
        a = set(a + b)
        return len(a)

    def intersect(self, nums1, nums2):
        nums1.sort(), nums2.sort()
        res = []
        it1, it2 = 0, 0
        while it1 < len(nums1) and it2 < len(nums2):
            if nums1[it1] < nums2[it2]:
                it1 += 1
            elif nums1[it1] > nums2[it2]:
                it2 += 1
            else:
                res += (nums1[it1],)
                it1 += 1
                it2 += 1
        return res
