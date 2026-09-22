# max sum level of the tree
class Node:
    def __init__(self, data):
        self.left = None
        self.right = None
        self.val = data


def maxSumLevel(root):
    if not root:
        return
    q = []
    q.append(root)
    q.append(None)
    curSum = maxSum = level = maxLevel = 0
    while len(q) > 0:
        node = q.pop(0)
        if node == None:
            if curSum > maxSum:
                maxSum = curSum
                maxLevel = level
            curSum = 0
            if len(q) > 0:
                q.append(None)
                level += 1
        else:
            curSum += node.val
            if node.left is not None:
                q.append(node.left)
            if node.right is not None:
                q.append(node.right)
    return maxLevel


if __name__ == "__main__":
    root = Node(2)
    root.left = Node(6)
    root.left.left = Node(3)
    root.left.left.left = Node(19)
    root.left.right = Node(1)
    root.left.left.right = Node(4)
    root.right = Node(8)
    root.right.left = Node(4)
    root.right.right = Node(2)

print(maxSumLevel(root))
