# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    # def inorder(self,root,res):
    #     if root is None:
    #         return
    #     self.inorder(root.left,res)
    #     res.append(root.val)
    #     self.inorder(root.right,res)

    def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        # res = []
        # self.inorder(root, res)
        # return res
        if root is None:
            return []
        q = []
        res = []
        while root or q:
            while root:
                q.append(root)
                root = root.left
            root = q.pop()
            res.append(root.val)
            root = root.right
        return res
