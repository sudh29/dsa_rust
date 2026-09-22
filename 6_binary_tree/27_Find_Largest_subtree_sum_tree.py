"""

definition of binary tree node.
class Node:
    def _init_(self,val):
        self.data = val
        self.left = None
        self.right = None
"""


class Solution:
    def findLargestSubtreeSum(self, root: Optional["Node"]) -> int:
        def solve(root, ans):
            if root is None:
                return 0
            ls = solve(root.left, ans)
            rs = solve(root.right, ans)
            curr_val = root.data + ls + rs
            ans[0] = max(ans[0], curr_val)
            return curr_val

        if root is None:
            return 0
        ans = [-999999999999]
        solve(root, ans)
        return ans[0]
