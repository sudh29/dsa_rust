class Solution:
    # Return the size of the largest sub-tree which is also a BST
    def largestBst(self, root):
        def is_bst(node, min_val=float("-inf"), max_val=float("inf")):
            if not node:
                return True
            if not (min_val < node.data < max_val):
                return False
            return is_bst(node.left, min_val, node.data) and is_bst(
                node.right, node.data, max_val
            )

        def count_nodes(node):
            if not node:
                return 0
            return 1 + count_nodes(node.left) + count_nodes(node.right)

        def find_largest_bst_size(node):
            if not node:
                return 0
            if is_bst(node):
                return count_nodes(node)
            return max(
                find_largest_bst_size(node.left), find_largest_bst_size(node.right)
            )

        return find_largest_bst_size(root)
