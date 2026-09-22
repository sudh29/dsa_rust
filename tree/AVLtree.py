# AVL tree


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None
        self.height = 1


class AVL_tree:
    def insert(self, root, key):
        if not root:
            return Node(key)
        elif key < root.val:
            root.left = self.insert(root.left, key)
        else:
            root.right = self.insert(root.right, key)

        root.height = 1 + max(self.getheight(root.left), self.getheight(root.right))

        balance = self.getbal(root)

        if balance > 1 and key < root.left.val:
            return self.RR(root)
        if balance > 1 and key > root.left.val:
            root.left = self.LR(root.left)
            return self.RR(root)

        if balance < -1 and key > root.right.val:
            return self.LR(root)
        if balance < -1 and key < root.right.val:
            root.right = self.RR(root.right)
            return self.LR(root)

        return root

    def LR(self, z):
        y = z.right
        z.right, y.left = y.left, z

        z.height = 1 + max(self.getheight(z.left), self.getheight(z.right))
        y.height = 1 + max(self.getheight(y.left), self.getheight(y.right))
        return y

    def RR(self, z):
        y = z.left
        z.left, y.right = y.right, z

        z.height = 1 + max(self.getheight(z.left), self.getheight(z.right))
        y.height = 1 + max(self.getheight(y.left), self.getheight(y.right))
        return y

    def getheight(self, root):
        if not root:
            return 0
        return root.height

    def getbal(self, root):
        if not root:
            return 0
        return self.getheight(root.left) - self.getheight(root.right)

    def preOrder(self, root):
        if not root:
            return
        print(root.val, end=" ")
        self.preOrder(root.left)
        self.preOrder(root.right)


myTree = AVL_tree()
root = None

root = myTree.insert(root, 10)
root = myTree.insert(root, 20)
root = myTree.insert(root, 30)
root = myTree.insert(root, 40)
root = myTree.insert(root, 50)
root = myTree.insert(root, 25)

# Preorder Traversal
print("Preorder of AVL tree")
myTree.preOrder(root)
print()
