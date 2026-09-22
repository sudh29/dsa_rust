"""
# Tree Node
class Node:
    def __init__(self, val):
        self.right = None
        self.data = val
        self.left = None
"""


class Solution:
    def binaryTreeToBST(self, root):
        def inorder_traversal(node):
            if node is not None:
                inorder_traversal(node.left)
                values.append(node.data)
                inorder_traversal(node.right)

        def convert_to_bst(node):
            nonlocal index
            if node is not None:
                convert_to_bst(node.left)
                node.data = values[index]
                index += 1
                convert_to_bst(node.right)

        values = []
        inorder_traversal(root)

        values.sort()

        index = 0
        convert_to_bst(root)
