class Node:
    def __init__(self, data):
        self.val = data
        self.left = None
        self.right = None


def maxLevel(root):
    if root == None:
        return 0
    return 1 + max(maxLevel(root.left), maxLevel(root.right))


sum = []


def maxLevelSum_(root, max_level, current):
    global sum
    if root == None:
        return
    sum[current] += root.val
    maxLevelSum_(root.left, max_level, current + 1)
    maxLevelSum_(root.right, max_level, current + 1)


def maxLevelSum(root):
    global sum
    max_level = maxLevel(root)
    i = 0
    sum = [None] * (max_level + 2)
    while i <= max_level + 1:
        sum[i] = 0
        i = i + 1
    maxLevelSum_(root, max_level, 1)
    maxSum = 0
    i = 1
    j = 1
    while i <= max_level:
        if maxSum < sum[i]:
            maxSum = sum[i]
            j = i
        i = i + 1
    return maxSum, j


if __name__ == "__main__":
    root = Node(1)
    root.left = Node(2)
    root.right = Node(3)
    root.left.left = Node(4)
    root.left.right = Node(5)
    root.right.right = Node(8)
    root.right.right.left = Node(6)
    root.right.right.right = Node(7)

    # Constructed Binary tree is:
    # 			 1
    # 			 / \
    # 			 2 3
    # 		 / \ \
    # 		 4 5 8
    # 				 / \
    # 				 6 7

    print(maxLevelSum(root))
