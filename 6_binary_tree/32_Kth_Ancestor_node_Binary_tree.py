ancestors = []


def kthAncestor(root, k, node_value):
    def dfs(node, target, path):
        global ancestors
        if not node:
            return
        path.append(node.data)
        if node.data == target:
            ancestors = path.copy()
        dfs(node.left, target, path)
        dfs(node.right, target, path)
        path.pop()

    path_val = []
    dfs(root, node_value, path_val)
    # print(ancestors)
    ancestors.pop()
    kth_ancestor = -1
    if k <= len(ancestors):
        kth_ancestor = ancestors[-k]

    return kth_ancestor
