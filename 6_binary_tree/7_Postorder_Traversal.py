# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def postorder(self, root, res):
        if root is None:
            return
        self.postorder(root.left, res)
        self.postorder(root.right, res)
        res.append(root.val)

    def postorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        # res = []
        # self.postorder(root, res)
        # return res
        if root is None:
            return []
        q = [root]
        res = []
        while q:
            root = q.pop()
            res.append(root.val)
            if root.left is not None:
                q.append(root.left)
            if root.right is not None:
                q.append(root.right)
        return res[::-1]
