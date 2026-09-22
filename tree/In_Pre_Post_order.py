# tree data structure with level order ,Inorder, Preorder,Postorder


class Node:
    def __init__(self, data):
        self.left = None
        self.right = None
        self.val = data


def Inorder(root):
    if root:
        Inorder(root.left)
        print(root.val, end=" ")
        Inorder(root.right)


def Preorder(root):
    if root:
        print(root.val, end=" ")
        Preorder(root.left)
        Preorder(root.right)


def Postorder(root):
    if root:
        Postorder(root.left)
        Postorder(root.right)
        print(root.val, end=" ")


def printLevelOrder(root):
    if not root:
        return
    q = []
    q.append(root)
    while len(q) > 0:
        node = q.pop(0)
        print(node.val, end=" ")
        if node.left is not None:
            q.append(node.left)
        if node.right is not None:
            q.append(node.right)


if __name__ == "__main__":
    root = Node(10)
    root.left = Node(11)
    root.left.left = Node(7)
    root.right = Node(9)
    root.right.left = Node(15)
    root.right.right = Node(8)
    print("Level Order")
    printLevelOrder(root)
    print()
    print("Inorder")
    Inorder(root)
    print()
    print("Preorder")
    Preorder(root)
    print()
    print("Postorder")
    Postorder(root)
    print()
