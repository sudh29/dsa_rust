class Solution:
    # Function to check whether a Binary Tree is BST or not.
    def isBST(self, root):
        temp = []

        def BST(root, temp):
            if root:
                BST(root.left, temp)
                temp.append(root.data)
                BST(root.right, temp)

        BST(root, temp)
        for i in range(1, len(temp)):
            if temp[i] <= temp[i - 1]:
                return False
        return True
