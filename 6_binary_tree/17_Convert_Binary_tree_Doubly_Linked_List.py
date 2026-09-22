'''
class Node:
    """ Class Node """
    def __init__(self, value):
        self.left = None
        self.data = value
        self.right = None
'''


# Function to convert a binary tree to doubly linked list.
class Solution:
    def bToDLL(self, root):
        if root is None:
            return None

        def solve(root, ans, head, prev):
            if root is None:
                return
            solve(root.left, ans, head, prev)
            ans.append(root.data)
            node = Node(root.data)
            if head[0] is None:
                head[0] = node
            else:
                prev[0].right = node
                node.left = prev[0]
            prev[0] = node
            solve(root.right, ans, head, prev)

        head = [None]
        prev = [None]
        ans = []  # inorder array
        solve(root, ans, head, prev)
        return head[0]
