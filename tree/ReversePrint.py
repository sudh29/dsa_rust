class Node:
    def __init__(self, data):
        self.left = None
        self.right = None
        self.val = data


def reversePrint(root):
    if not root:
        return
    q = []
    stack = []
    q.append(root)
    while len(q) > 0:
        node = q.pop(0)
        stack.append(node)
        if node.right is not None:
            q.append(node.right)
        if node.left is not None:
            q.append(node.left)
    while len(stack) > 0:
        print(stack.pop().val, end=" ")
    print()


if __name__ == "__main__":
    root = Node(1)
    root.left = Node(2)
    root.left.left = Node(4)
    root.left.right = Node(5)
    root.right = Node(3)
    root.right.left = Node(6)
    root.right.right = Node(7)

print(reversePrint(root))
