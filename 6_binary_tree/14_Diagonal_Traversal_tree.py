"""
# Node Class:
class Node:
    def _init_(self,val):
        self.data = val
        self.left = None
        self.right = None
"""


# Complete the function below
class Solution:
    def diagonal(self, root):
        if root is None:
            return
        res = []
        left_q = []
        node = root
        while node:
            res.append(node.data)
            if node.left:
                left_q.insert(0, node.left)

            if node.right:
                node = node.right
            else:
                node = left_q.pop() if left_q else None
        return res
