"""
class Node:
    def __init__(self, val):
        self.right = None
        self.data = val
        self.left = None
"""

# your task is to complete this function
# function should return True is Tree is SumTree else return False

res = False


class Solution:
    def isSumTree(self, root):
        if root is None or (root.left is None and root.right is None):
            return True
        global res
        res = False

        def solve(node):
            global res
            if node is None:
                return 0
            if node.left is None and node.right is None:
                return node.data
            ls = solve(node.left)
            rs = solve(node.right)
            if (ls + rs) != node.data:
                res = True
            return ls + rs + node.data

        value = solve(root)
        if res:
            return False
        return False if 2 * root.data != value else True
