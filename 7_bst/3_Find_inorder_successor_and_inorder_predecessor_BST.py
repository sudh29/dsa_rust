"""
class Node:
    def __init__(self, key):
        self.key = key
        self.left = None
        self.right = None
"""


# This function finds predecessor and successor of key in BST.
# It sets pre and suc as predecessor and successor respectively
class Solution:
    def findPreSuc(self, root, pre, suc, key):
        if root is None:
            return
        if root.key == key:
            if root.left:
                temp = root.left
                while temp.right:
                    temp = temp.right
                pre.key = temp.key
            if root.right:
                temp = root.right
                while temp.left:
                    temp = temp.left
                suc.key = temp.key
        elif root.key < key:
            pre.key = root.key
            self.findPreSuc(root.right, pre, suc, key)
        else:
            suc.key = root.key
            self.findPreSuc(root.left, pre, suc, key)
