class Solution:
    def buildBalancedTree(self, root):
        def inorder(node, ans):
            if node is None:
                return
            inorder(node.left, ans)
            ans.append(node)
            inorder(node.right, ans)

        def build_balanced_tree(nodes, st, end):
            if st > end:
                return None
            mid = (st + end) // 2
            node = nodes[mid]
            node.left = build_balanced_tree(nodes, st, mid - 1)
            node.right = build_balanced_tree(nodes, mid + 1, end)
            return node

        inorder_arr = []
        inorder(root, inorder_arr)
        n = len(inorder_arr)
        # print(inorder_arr)
        return build_balanced_tree(inorder_arr, 0, n - 1)
