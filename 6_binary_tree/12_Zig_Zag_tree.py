"""
class Node:
    def __init__(self,val):
        self.data = val
        self.left = None
        self.right = None
"""


class Solution:
    # Function to store the zig zag order traversal of tree in a list.
    def zigZagTraversal(self, root):
        if root is None:
            return
        flag = False
        q = [root]
        res = []
        while q:
            n = len(q)
            level = []
            for i in range(n):
                curr = q.pop(0)
                level.append(curr.data)
                if curr.left:
                    q.append(curr.left)
                if curr.right:
                    q.append(curr.right)
            flag = not flag
            if flag == False:
                level = level[::-1]
            for i in level:
                res.append(i)
        return res
