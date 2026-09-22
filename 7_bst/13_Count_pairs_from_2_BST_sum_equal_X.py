"""
# Tree Node
class Node:
    def __init__(self, val):
        self.right = None
        self.data = val
        self.left = None
"""


class Solution:
    # def find(self,node,k):
    #     if node is None:
    #         return 0
    #     if node.data==k:
    #         return 1
    #     elif node.data>k:
    #         return self.find(node.left,k)
    #     else:
    #         return self.find(node.right,k)

    # def countPairs(self, root1, root2, x):
    #     if root1 is None:
    #         return 0
    #     return self.find(root2,x-root1.data) + self.countPairs(root1.left,root2,x) + self.countPairs(root1.right,root2,x)

    def inorder(self, node, ans):
        if node is None:
            return
        self.inorder(node.left, ans)
        ans.append(node.data)
        self.inorder(node.right, ans)

    def countPairs(self, root1, root2, x):
        r1 = []
        r2 = []
        self.inorder(root1, r1)
        self.inorder(root2, r2)

        i, j, count = 0, len(r2) - 1, 0

        while i < len(r1) and j >= 0:
            if r1[i] + r2[j] == x:
                count += 1
                i += 1
                j -= 1
            elif r1[i] + r2[j] < x:
                i += 1
            else:
                j -= 1
        return count
