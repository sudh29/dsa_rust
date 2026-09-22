def printLeftBoundary(root, res):
    curr = root.left
    while curr:
        if not isleaf(curr):
            res.append(curr.data)
        if curr.left:
            curr = curr.left
        else:
            curr = curr.right


def printRightBoundary(root, res):
    curr = root.right
    st = []
    while curr:
        if not isleaf(curr):
            st.append(curr.data)
        if curr.right:
            curr = curr.right
        else:
            curr = curr.left
    res.extend(reversed(st))


def printLeaves(root, res):
    if isleaf(root):
        res.append(root.data)
    if root.left:
        printLeaves(root.left, res)
    if root.right:
        printLeaves(root.right, res)


def isleaf(root):
    return not root.left and not root.right


"""
class Node:
    def __init__(self, val):
        self.right = None
        self.data = val
        self.left = None
"""


class Solution:
    def printBoundaryView(self, root):
        if not root:
            return res
        res = []
        if not isleaf(root):
            res.append(root.data)

        printLeftBoundary(root, res)
        printLeaves(root, res)
        printRightBoundary(root, res)

        return res
