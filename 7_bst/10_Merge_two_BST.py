"""
class Node:
    def __init__(self, val):
        self.right = None
        self.data = val
        self.left = None
"""


class Solution:
    def merge(self, root1, root2):
        def merge_sorted_arr(arr1, arr2):
            arr = []
            i = j = 0
            while i < len(arr1) and j < len(arr2):
                if arr1[i] <= arr2[j]:
                    arr.append(arr1[i])
                    i += 1
                else:
                    arr.append(arr2[j])
                    j += 1
            while i < len(arr1):
                arr.append(arr1[i])
                i += 1
            while j < len(arr2):
                arr.append(arr2[j])
                j += 1
            return arr

        def arr_to_bst(arr):
            if not arr:
                return None
            mid = len(arr) // 2
            root = Node(arr[mid])
            root.left = arr_to_bst(arr[:mid])
            root.right = arr_to_bst(arr[mid + 1 :])
            return root

        def inorder(node, ans):
            if node is None:
                return
            inorder(node.left, ans)
            ans.append(node.data)
            inorder(node.right, ans)

        tree1 = []
        tree2 = []
        inorder(root1, tree1)
        inorder(root2, tree2)
        tree = merge_sorted_arr(tree1, tree2)
        # print(tree)
        # root = arr_to_bst(tree)
        return tree
