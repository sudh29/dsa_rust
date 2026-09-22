"""
# Node Class:
class Node:
    def _init_(self,val):
        self.data = val
        self.left = None
        self.right = None
"""


class Solution:
    def dupSub(self, root):
        # seen_subtrees = set()
        # def serialize_and_check_duplicate(node):
        #     if not node: return ''
        #     if not node.left and not node.right: return str(node.data)

        #     left_serialized = serialize_and_check_duplicate(node.left)
        #     if left_serialized == True:
        #         return True

        #     right_serialized = serialize_and_check_duplicate(node.right)
        #     if right_serialized == True:
        #         return True

        #     current_serialized = str(node.data)
        #     if left_serialized != '':
        #         current_serialized += 'l' + left_serialized
        #     if right_serialized != '':
        #         current_serialized += 'r' + right_serialized

        #     if current_serialized in seen_subtrees:
        #         return True
        #     seen_subtrees.add(current_serialized)
        #     return current_serialized

        # return 1 if serialize_and_check_duplicate(root) == True else 0

        subtree_map = {}
        res = []

        def dfs(node):
            if not node:
                return "null"
            if not node.left and not node.right:
                s = str(node.data)
                return s
            s = ",".join([str(node.data), dfs(node.left), dfs(node.right)])

            if s in subtree_map:
                if subtree_map[s] == 1:
                    res.append(node)
                subtree_map[s] += 1
            else:
                subtree_map[s] = 1
            return s

        dfs(root)
        return 1 if len(res) else 0
