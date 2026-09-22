"""
class Node:
    def __init__(self,val):
        self.data = val
        self.left = None
        self.right = None
"""


def count_nodes(root):
    if root is None:
        return 0
    else:
        return 1 + count_nodes(root.left) + count_nodes(root.right)


def complete_tree_util(root, index, node_count):
    if root is None:
        return True
    if index >= node_count:
        return False
    return complete_tree_util(
        root.left, 2 * index + 1, node_count
    ) and complete_tree_util(root.right, 2 * index + 2, node_count)


def heap_property_util(root):
    if root.left is None and root.right is None:
        return True
    if root.right is None:
        return root.data >= root.left.data
    else:
        if root.data >= root.left.data and root.data >= root.right.data:
            return heap_property_util(root.left) and heap_property_util(root.right)
        else:
            return False


class Solution:
    # Your Function Should return True/False
    def isHeap(self, root):
        node_count = count_nodes(root)
        if complete_tree_util(root, 0, node_count) and heap_property_util(root):
            return True
        else:
            return False
