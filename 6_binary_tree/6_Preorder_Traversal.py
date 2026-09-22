# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def preorder(self, root, res):
        if root is None:
            return
        res.append(root.val)
        self.preorder(root.left, res)
        self.preorder(root.right, res)

    def preorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        # res = []
        # self.preorder(root,res)
        # return res

        if root is None:
            return
        q = [root]
        res = []
        while q:
            curr = q.pop()
            res.append(curr.val)
            if curr.right is not None:
                q.append(curr.right)
            if curr.left is not None:
                q.append(curr.left)
        return res
