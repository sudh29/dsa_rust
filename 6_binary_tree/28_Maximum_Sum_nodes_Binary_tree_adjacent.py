"""
# Node Class:
class Node:
    def __init__(self,val):
        self.data = val
        self.left = None
        self.right = None
"""

max_res = -999999999
val_dic = {}


class Solution:
    # Function to return the maximum sum of non-adjacent nodes.
    def getMaxSum(self, root):
        # global max_res
        # if root is None:
        #     return 0
        # if val_dic.get(root,None): return val_dic[root] # DP
        # inc = root.data
        # if root.left:
        #     inc += self.getMaxSum(root.left.left)
        #     inc += self.getMaxSum(root.left.right)
        # if root.right:
        #     inc += self.getMaxSum(root.right.left)
        #     inc += self.getMaxSum(root.right.right)

        # exc = self.getMaxSum(root.left) + self.getMaxSum(root.right)
        # val_dic[root] = max(max_res,max(inc,exc))

        # return val_dic[root]

        def dfs(node):
            if not node:
                return (0, 0)
            left_incl, left_excl = dfs(node.left)
            right_incl, right_excl = dfs(node.right)
            incl_sum = node.data + left_excl + right_excl
            excl_sum = max(left_incl, left_excl) + max(right_incl, right_excl)
            return (incl_sum, excl_sum)

        root_incl, root_excl = dfs(root)
        return max(root_incl, root_excl)
