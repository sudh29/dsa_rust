"""
# Node class

class Node:
    def __init__(self,val):
        self.data = val
        self.right = None
        self.left = None

"""


class Solution:
    def buildtree(self, inorder, preorder, n):
        def buildTree(preorder, inorder):
            if not preorder or not inorder:
                return None
            root_val = preorder[0]
            root = Node(root_val)
            root_index = inorder.index(root_val)
            root.left = buildTree(preorder[1 : 1 + root_index], inorder[:root_index])
            root.right = buildTree(
                preorder[1 + root_index :], inorder[root_index + 1 :]
            )
            return root

        root = buildTree(preorder, inorder)
        return root
