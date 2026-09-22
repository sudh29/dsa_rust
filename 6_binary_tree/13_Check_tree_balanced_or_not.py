"""class Node:
# Constructor to create a new Node
def __init__(self, data):
    self.data = data
    self.left = None
    self.right = None"""


# Function to check whether a binary tree is balanced or not.
class Solution:
    def isBalanced(self, root):
        if root is None:
            return True

        lh = self.isBalanced(root.left)
        if lh == 0:
            return False
        rh = self.isBalanced(root.right)
        if rh == 0:
            return False
        if abs(lh - rh) > 1:
            return False
        else:
            return max(lh, rh) + 1
