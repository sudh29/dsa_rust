# Insert and Delete in tree


class Node:
    def __init__(self, data):
        self.left = None
        self.right = None
        self.val = data


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


def insert(root, data):
    if not root:
        root = Node(data)
        return
    q = []
    q.append(root)
    while len(q):
        root = q.pop(0)
        if not root.left:
            root.left = Node(data)
            break
        else:
            q.append(root.left)

        if not root.right:
            root.right = Node(data)
            break
        else:
            q.append(root.right)


def deleteDeepest(root, data):
    q = []
    q.append(root)
    while len(q):
        temp = q.pop(0)
        if temp is data:
            temp = None
            return
        if temp.right:
            if temp.right is data:
                temp.right = None
                return
            else:
                q.append(temp.right)
        if temp.left:
            if temp.left is data:
                temp.left = None
                return
            else:
                q.append(temp.left)


# function to delete element in binary tree
def deletion(root, data):
    if root == None:
        return None
    if root.left == None and root.right == None:
        if root.val == data:
            return None
        else:
            return root
    key_node = None
    q = []
    q.append(root)
    while len(q):
        temp = q.pop(0)
        if temp.val == data:
            key_node = temp
        if temp.left:
            q.append(temp.left)
        if temp.right:
            q.append(temp.right)
    if key_node:
        x = temp.val
        deleteDeepest(root, temp)
        key_node.val = x
    return root


if __name__ == "__main__":
    root = Node(10)
    root.left = Node(11)
    root.left.left = Node(7)
    root.right = Node(9)
    root.right.left = Node(15)
    root.right.right = Node(8)

    key = 12
    insert(root, key)
    printLevelOrder(root)
    print()

    key = 11
    root = deletion(root, key)
    printLevelOrder(root)
    print()
