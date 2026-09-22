# class Node:
#     def __init__(self, val):
#         self.data = val
#         self.left = None
#         self.right = None


# return the Kth largest element in the given BST rooted at 'root'
class Solution:
    def kthLargest(self, root, k):
        def kth(node):
            nonlocal k, result
            if node is None or k == 0:
                return
            kth(node.right)
            k -= 1
            if k == 0:
                result = node.data
            kth(node.left)

        result = None
        kth(root)
        return result
