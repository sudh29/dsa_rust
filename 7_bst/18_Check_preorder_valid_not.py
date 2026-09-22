class Node:
    def __init__(self, data=0):
        self.data = data
        self.left = None
        self.right = None


# Function that constructs BST from its preorder traversal.
def post_order(pre, size) -> Node:
    def solve(pre):
        if not pre:
            return []
        root = pre[0]
        left_subtree = [x for x in pre if x < root]
        right_subtree = [x for x in pre if x > root]

        left_postorder = solve(left_subtree)
        right_postorder = solve(right_subtree)

        return left_postorder + right_postorder + [root]

    def bst_from_preorder(preorder):
        if not preorder:
            return None
        root_val = preorder[0]
        root = Node(root_val)
        i = 1
        while i < len(preorder) and preorder[i] < root_val:
            i += 1
        root.left = bst_from_preorder(preorder[1:i])
        root.right = bst_from_preorder(preorder[i:])
        return root

    postorder = solve(pre)
    # print(postorder)
    return bst_from_preorder(pre)
