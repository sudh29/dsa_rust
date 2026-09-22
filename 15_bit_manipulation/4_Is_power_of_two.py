# User function Template for python3


class Solution:
    ##Complete this function
    # Function to check if given number n is a power of two.
    def isPowerofTwo(self, n):
        ##Your code here
        if n == 0:
            return False
        else:
            res = n & (n - 1)
            return True if res == 0 else False

    def isPowerofTwo(self, n):
        if n == 1 or n == 2:
            return True
        elif n % 2 == 0:
            x = 1
            while x < n:
                x = x * 2
                if x == n:
                    return True
                if x > n:
                    return False
        else:
            return False
