# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def constructPreorderTree(self, node, x):
        if node.val > x:
            if node.left is None:
                node.left = TreeNode(x)
                return
            self.constructPreorderTree(node.left, x)
        if node.val < x:
            if node.right is None:
                node.right = TreeNode(x)
                return
            self.constructPreorderTree(node.right, x)

    def bstFromPreorder(self, preorder: List[int]) -> Optional[TreeNode]:
        n = len(preorder)
        if n == 0:
            return None
        else:
            root = TreeNode(preorder[0])
            for i in preorder[1:]:
                self.constructPreorderTree(root, i)
            return root
