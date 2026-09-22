"""
class Node:
    def __init__(self,val):
        self.data=val
        self.left=None
        self.right=None
"""


class Solution:
    def sumOfLongRootToLeafPath(self, root):
        def solve(root, level, sum_val, maxLen, maxSum):
            if root is None:
                if level > maxLen[0]:
                    maxSum[0] = sum_val
                    maxLen[0] = level
                elif level == maxLen[0]:
                    maxSum[0] = max(maxSum[0], sum_val)
                return
            solve(root.left, level + 1, sum_val + root.data, maxLen, maxSum)
            solve(root.right, level + 1, sum_val + root.data, maxLen, maxSum)

        if root is None:
            return 0
        maxSum = [-999999999999]
        maxLen = [0]
        solve(root, 0, 0, maxLen, maxSum)
        return maxSum[0]
