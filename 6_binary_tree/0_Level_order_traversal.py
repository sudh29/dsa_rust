class Solution:
    # Function to return the level order traversal of a tree.
    def levelOrder(self, root):
        # Code here
        res = []
        queue = [root]
        while queue:
            value = queue.pop(0)
            res.append(value.data)
            if value.left:
                queue.append(value.left)
            if value.right:
                queue.append(value.right)
        return res
