"""
class Node:
    def __init__(self, value):
        self.left = None
        self.data = value
        self.right = None
"""


class Solution:
    # Function to return the lowest common ancestor in a Binary Tree.
    def lca(self, root, n1, n2):
        # Code here
        # if root is None:
        #     return None
        # if root.data==n1 or root.data==n2:
        #     return root
        # ls=self.lca(root.left,n1,n2)
        # rs=self.lca(root.right,n1,n2)

        # if ls and rs:
        #     return root
        # return ls if ls else rs

        def solve(root, path, k):
            if root is None:
                return False
            path.append(root)
            if root.data == k:
                return True
            if (root.left != None and solve(root.left, path, k)) or (
                root.right != None and solve(root.right, path, k)
            ):
                return True
            path.pop()
            return False

        path1 = []
        path2 = []
        if not solve(root, path1, n1) or not solve(root, path2, n2):
            return -1
        i = 0
        while i < len(path1) and i < len(path2):
            if path1[i] != path2[i]:
                break
            i += 1
        return path1[i - 1]
