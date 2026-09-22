class Node:
    def __init__(self, data):
        self.left = None
        self.right = None
        self.val = data


def maxElement(root):
    if not root:
        return
    q = []
    q.append(root)
    maxElement = float("-inf")
    while len(q) > 0:
        node = q.pop(0)
        # check element greater than max till now
        if maxElement < node.val:
            maxElement = node.val

        if node.left is not None:
            q.append(node.left)
        if node.right is not None:
            q.append(node.right)
    return maxElement


if __name__ == "__main__":
    root = Node(2)
    root.left = Node(6)
    root.left.left = Node(3)
    root.left.left.left = Node(11)
    root.left.right = Node(1)
    root.left.left.right = Node(4)
    root.right = Node(8)
    root.right.left = Node(4)
    root.right.right = Node(2)

print(maxElement(root))
