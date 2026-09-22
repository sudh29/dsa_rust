"""
class Node:
    def __init__(self, val):
        self.right = None
        self.data = val
        self.left = None
"""


class Solution:
    # Return the Kth smallest element in the given BST
    def KthSmallestElement(self, root, k):
        def kth(node):
            nonlocal k, res
            if node is None or k == 0:
                return
            kth(node.left)
            k -= 1
            if k == 0:
                res = node.data
            kth(node.right)

        res = -1
        kth(root)
        return res
