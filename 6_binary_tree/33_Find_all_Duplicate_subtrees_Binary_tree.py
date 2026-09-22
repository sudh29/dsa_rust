"""
class Node:
    def __init__(self,val):
        self.data=val
        self.left=None
        self.right=None
"""


class Solution:
    def printAllDups(self, root):
        subtree_map = {}
        res = []

        def dfs(node):
            if not node:
                return "null"
            s = ",".join([str(node.data), dfs(node.left), dfs(node.right)])

            if s in subtree_map:
                if subtree_map[s] == 1:
                    res.append(node)
                subtree_map[s] += 1
            else:
                subtree_map[s] = 1
            return s

        dfs(root)
        res = sorted(res, key=lambda x: x.data)
        return res
