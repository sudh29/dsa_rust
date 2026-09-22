"""
class Node:
    def __init__(self, val):
        self.right = None
        self.data = val
        self.left = None
"""


# Function to find the minimum element in the given BST.
def minValue(root):
    # if root is None:
    #     return -1
    # else:
    #     if root.left:
    #         return minValue(root.left)
    #     else:
    #         return root.data

    if root is None:
        return -1
    q = [root]
    while q:
        curr = q.pop()
        if curr.left:
            q.append(curr.left)
        else:
            return curr.data
    return curr.data
