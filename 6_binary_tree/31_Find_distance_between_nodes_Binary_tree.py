"""
# Node Class:
class Node:
    def __init__(self,val):
        self.data = val
        self.left = None
        self.right = None
"""


class Solution:
    def findDist(self, root, a, b):
        def lca(root, n1, n2):
            if root is None:
                return None
            if root.data == n1 or root.data == n2:
                return root
            ls = lca(root.left, n1, n2)
            rs = lca(root.right, n1, n2)
            if ls and rs:
                return root
            return ls if ls else rs

        def solve(root, k):
            if root is None:
                return 0
            if root.data == k:
                return 1
            ls = solve(root.left, k)
            rs = solve(root.right, k)
            if ls == 0 and rs == 0:
                return 0
            return ls + rs + 1

        if root is None:
            return 0
        lca_root = lca(root, a, b)
        left = solve(lca_root, a)
        right = solve(lca_root, b)
        return left + right - 2
