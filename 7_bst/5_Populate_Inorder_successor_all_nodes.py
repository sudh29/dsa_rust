class Node:
    def __init__(self, val):
        self.right = None
        self.data = val
        self.left = None
        self.next = None


temp = Node(None)


class Solution:
    def populateNext(self, root):
        global temp
        if root is None:
            return
        self.populateNext(root.left)
        if temp != None:
            temp.next = root
        temp = root
        self.populateNext(root.right)
