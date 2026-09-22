class TreeNode:
    def __init__(self, value):
        self.val = value
        self.left = None
        self.right = None


def flatten_bst(root):
    def inorder(curr, prev):
        if curr is None:
            return
        inorder(curr.left, prev)
        prev[0].left = None
        prev[0].right = curr
        prev[0] = curr
        inorder(curr.right, prev)

    dummy = TreeNode(None)
    prev = [dummy]
    # print(dummy==prev[0])
    inorder(root, prev)
    # print(dummy==prev[0])
    prev[0].left = None
    prev[0].right = None
    result = dummy.right
    return result


# Example usage:
# Construct a BST
root = TreeNode(8)
root.left = TreeNode(5)
root.right = TreeNode(10)
root.left.left = TreeNode(1)
root.left.right = TreeNode(7)
root.right.right = TreeNode(12)

# Flatten the BST to a sorted list
sorted_list = flatten_bst(root)
while sorted_list.right is not None:
    (print(sorted_list.val),)  # Output: [1, 5, 7, 8, 10, 12]
    sorted_list = sorted_list.right
print(sorted_list.val)
