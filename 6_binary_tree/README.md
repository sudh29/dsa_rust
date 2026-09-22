# Binary Tree Problems with Patterns

| File | Pattern(s) |
|------|------------|
| [p00_level_order_traversal.rs](p00_level_order_traversal.rs) | BFS / Queue |
| [p01_reverse_level_order_traversal.rs](p01_reverse_level_order_traversal.rs) | BFS / Stack |
| [p02_height_of_a_tree.rs](p02_height_of_a_tree.rs) | Recursion / DFS |
| [p03_diameter_of_a_tree.rs](p03_diameter_of_a_tree.rs) | DFS / Recursion |
| [p04_mirror_of_a_tree.rs](p04_mirror_of_a_tree.rs) | Recursion / Tree Manipulation |
| [p05_inorder_traversal.rs](p05_inorder_traversal.rs) | DFS / Inorder Traversal |
| [p06_preorder_traversal.rs](p06_preorder_traversal.rs) | DFS / Preorder Traversal |
| [p07_postorder_traversal.rs](p07_postorder_traversal.rs) | DFS / Postorder Traversal |
| [p08_left_view_tree.rs](p08_left_view_tree.rs) | BFS / Level Order with First Node |
| [p09_right_view_tree.rs](p09_right_view_tree.rs) | BFS / Level Order with Last Node |
| [p10_top_view_tree.rs](p10_top_view_tree.rs) | BFS / Horizontal Distance Map |
| [p11_bottom_view_tree.rs](p11_bottom_view_tree.rs) | BFS / Last Node at Each HD |
| [p12_zig_zag_tree.rs](p12_zig_zag_tree.rs) | BFS / Level Order with Direction Toggle |
| [p13_check_tree_balanced_or_not.rs](p13_check_tree_balanced_or_not.rs) | DFS / Height Check |
| [p14_diagonal_traversal_tree.rs](p14_diagonal_traversal_tree.rs) | Hashing / Queue |
| [p15_boundary_traversal_tree.rs](p15_boundary_traversal_tree.rs) | DFS + BFS / Boundary Collection |
| [p16_construct_binary_tree_string_bracket_representation.rs](p16_construct_binary_tree_string_bracket_representation.rs) | DFS / String Construction |
| [p17_convert_binary_tree_doubly_linked_list.rs](p17_convert_binary_tree_doubly_linked_list.rs) | Inorder Traversal / DLL Formation |
| [p18_convert_binary_tree_sum_tree.rs](p18_convert_binary_tree_sum_tree.rs) | Postorder DFS / Subtree Sum |
| [p19_construct_binary_tree_from_inorder_and_preorder_traversal.rs](p19_construct_binary_tree_from_inorder_and_preorder_traversal.rs) | Recursion / Tree Reconstruction |
| [p20_find_minimum_swaps_required_convert_binary_tree_into_bst.rs](p20_find_minimum_swaps_required_convert_binary_tree_into_bst.rs) | Inorder Traversal / Sorting |
| [p21_check_if_binary_tree_is_sum_tree_or_not.rs](p21_check_if_binary_tree_is_sum_tree_or_not.rs) | Recursion / Subtree Sum Validation |
| [p22_leaf_at_same_leve.rs](p22_leaf_at_same_leve.rs) | BFS / Level Consistency |
| [p23_check_binary_tree_duplicate_subtrees.rs](p23_check_binary_tree_duplicate_subtrees.rs) | DFS / Hashing Subtrees |
| [p24_check_mirror_n_ary_tree.rs](p24_check_mirror_n_ary_tree.rs) | Recursion / Symmetry Check |
| [p25_sum_nodes_longest_path_from_root_leaf_node.rs](p25_sum_nodes_longest_path_from_root_leaf_node.rs) | DFS / Depth + Path Sum |
| [p26_check_graph_tree_or_not.rs](p26_check_graph_tree_or_not.rs) | Graph / Cycle Detection |
| [p27_find_largest_subtree_sum_tree.rs](p27_find_largest_subtree_sum_tree.rs) | Postorder DFS / Subtree Sum |
| [p28_maximum_sum_nodes_binary_tree_adjacent.rs](p28_maximum_sum_nodes_binary_tree_adjacent.rs) | Tree DP / Include-Exclude |
| [p29_print_all_k_sum_paths_binary_tree.rs](p29_print_all_k_sum_paths_binary_tree.rs) | DFS / Backtracking |
| [p30_find_lca_binary_tree.rs](p30_find_lca_binary_tree.rs) | DFS / Recursion |
| [p31_find_distance_between_nodes_binary_tree.rs](p31_find_distance_between_nodes_binary_tree.rs) | LCA + Depth Count |
| [p32_kth_ancestor_node_binary_tree.rs](p32_kth_ancestor_node_binary_tree.rs) | DFS / Backtracking |
| [p33_find_all_duplicate_subtrees_binary_tree.rs](p33_find_all_duplicate_subtrees_binary_tree.rs) | DFS / Subtree Serialization |
| [p34_tree_isomorphism_problem.rs](p34_tree_isomorphism_problem.rs) | Recursion / Tree Comparison |


In a tree structure, Depth First Search (DFS) and Breadth First Search (BFS) algorithms are used to traverse the nodes. Here's how they work:

# Depth First Search (DFS):

DFS explores as far as possible along each branch before backtracking. It starts at the root node and explores as far as possible along each branch before backtracking.
There are three types of DFS: Pre-order, In-order, and Post-order.
Pre-order: Visit the current node, then recursively visit the left subtree, and then recursively visit the right subtree.
In-order: Recursively visit the left subtree, visit the current node, and then recursively visit the right subtree.
Post-order: Recursively visit the left subtree, recursively visit the right subtree, and then visit the current node.

# Breadth First Search (BFS):

BFS explores neighbors of the current vertex before moving to the next level of vertices.
It starts at the root node and explores all the neighbor nodes at the present depth before moving on to the nodes at the next depth level.
BFS uses a queue data structure to keep track of the nodes to be visited next.

## Below are the Python implementations of DFS and BFS for a binary tree:

```python
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

# Depth First Search (DFS) implementations
def dfs_preorder(node):
    if node is None:
        return
    print(node.val, end=' ')
    dfs_preorder(node.left)
    dfs_preorder(node.right)

def dfs_inorder(node):
    if node is None:
        return
    dfs_inorder(node.left)
    print(node.val, end=' ')
    dfs_inorder(node.right)

def dfs_postorder(node):
    if node is None:
        return
    dfs_postorder(node.left)
    dfs_postorder(node.right)
    print(node.val, end=' ')

# Breadth First Search (BFS) implementation
def bfs(root):
    if root is None:
        return
    queue = [root]
    while queue:
        node = queue.pop(0)
        print(node.val, end=' ')
        if node.left:
            queue.append(node.left)
        if node.right:
            queue.append(node.right)

# Example usage:
# Creating a sample binary tree
root = TreeNode(1)
root.left = TreeNode(2)
root.right = TreeNode(3)
root.left.left = TreeNode(4)
root.left.right = TreeNode(5)

# Output of different DFS traversals
print("DFS Pre-order:")
dfs_preorder(root)  # Output: 1 2 4 5 3
print("\nDFS In-order:")
dfs_inorder(root)   # Output: 4 2 5 1 3
print("\nDFS Post-order:")
dfs_postorder(root) # Output: 4 5 2 3 1

# Output of BFS traversal
print("\nBFS:")
bfs(root)  # Output: 1 2 3 4 5
```
