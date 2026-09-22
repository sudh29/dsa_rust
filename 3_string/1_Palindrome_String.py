class Solution:
	def isPalindrome(self, S):
		# code here
		n=len(S)
		for i in range(n//2):
		    if S[i]!=S[n-i-1]:
		        return 0
	    return 1
