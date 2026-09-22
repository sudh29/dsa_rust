"""
# Node Class:
class Node:
    def __init__(self,val):
        self.data = val
        self.left = None
        self.right = None
"""


class Solution:
    def toSumTree(self, root):
        if root is None:
            return 0
        old_root_data = root.data
        root.data = self.toSumTree(root.left) + self.toSumTree(root.right)
        return old_root_data + root.data


class Solution:
    def sum_child(self, node):
        left_sum = node.left.data if node.left else 0
        right_sum = node.right.data if node.right else 0
        return left_sum + right_sum

    def toSumTree(self, root):
        # code here
        if root is None:
            return 0
        old_val = root.data
        left_sum = self.toSumTree(root.left)
        right_sum = self.toSumTree(root.right)
        root.data = left_sum + right_sum
        return root.data + old_val

        # if root is None:
        #     return
        # q=[root]
        # while q:
        #     curr = q.pop(0)
        #     old_val = curr.data
        #     curr.data = self.sum_child(curr)
        #     if curr.left: q.append(curr.left)
        #     if curr.right: q.append(curr.right)
