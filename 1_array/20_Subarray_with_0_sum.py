class Solution:
    # Function to check whether there is a subarray present with 0-sum or not.
    def subArrayExists(self, arr, n):
        ##Your code here
        # Return true or false
        sum_val = 0
        s = set()
        for i in range(n):
            sum_val += arr[i]
            if sum_val == 0 or sum_val in s:
                return True
            s.add(sum_val)
        return False
