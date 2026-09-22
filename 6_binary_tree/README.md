# Binary Tree Problems with Patterns

| File | Pattern(s) |
|------|------------|
| [0_Level_order_traversal.py](0_Level_order_traversal.py) | BFS / Queue |
| [1_Reverse_Level_Order_traversal.py](1_Reverse_Level_Order_traversal.py) | BFS / Stack |
| [2_Height_of_a_tree.py](2_Height_of_a_tree.py) | Recursion / DFS |
| [3_Diameter_of_a_tree.py](3_Diameter_of_a_tree.py) | DFS / Recursion |
| [4_Mirror_of_a_tree.py](4_Mirror_of_a_tree.py) | Recursion / Tree Manipulation |
| [5_Inorder_Traversal.py](5_Inorder_Traversal.py) | DFS / Inorder Traversal |
| [6_Preorder_Traversal.py](6_Preorder_Traversal.py) | DFS / Preorder Traversal |
| [7_Postorder_Traversal.py](7_Postorder_Traversal.py) | DFS / Postorder Traversal |
| [8_Left_View_tree.py](8_Left_View_tree.py) | BFS / Level Order with First Node |
| [9_Right_View_Tree.py](9_Right_View_Tree.py) | BFS / Level Order with Last Node |
| [10_Top_View_tree.py](10_Top_View_tree.py) | BFS / Horizontal Distance Map |
| [11_Bottom_View_tree.py](11_Bottom_View_tree.py) | BFS / Last Node at Each HD |
| [12_Zig_Zag_tree.py](12_Zig_Zag_tree.py) | BFS / Level Order with Direction Toggle |
| [13_Check_tree_balanced_or_not.py](13_Check_tree_balanced_or_not.py) | DFS / Height Check |
| [14_Diagonal_Traversal_tree.py](14_Diagonal_Traversal_tree.py) | Hashing / Queue |
| [15_Boundary_traversal_tree.py](15_Boundary_traversal_tree.py) | DFS + BFS / Boundary Collection |
| [16_Construct_Binary_Tree_String_Bracket_Representation.py](16_Construct_Binary_Tree_String_Bracket_Representation.py) | DFS / String Construction |
| [17_Convert_Binary_tree_Doubly_Linked_List.py](17_Convert_Binary_tree_Doubly_Linked_List.py) | Inorder Traversal / DLL Formation |
| [18_Convert_Binary_tree_Sum_tree.py](18_Convert_Binary_tree_Sum_tree.py) | Postorder DFS / Subtree Sum |
| [19_Construct_Binary_tree_from_Inorder_and_preorder_traversal.py](19_Construct_Binary_tree_from_Inorder_and_preorder_traversal.py) | Recursion / Tree Reconstruction |
| [20_Find_minimum_swaps_required_convert_Binary_tree_into_BST.py](20_Find_minimum_swaps_required_convert_Binary_tree_into_BST.py) | Inorder Traversal / Sorting |
| [21_Check_if_Binary_tree_is_Sum_tree_or_not.py](21_Check_if_Binary_tree_is_Sum_tree_or_not.py) | Recursion / Subtree Sum Validation |
| [22_Leaf_at_same_leve.py](22_Leaf_at_same_leve.py) | BFS / Level Consistency |
| [23_Check_Binary_Tree_duplicate_subtrees.py](23_Check_Binary_Tree_duplicate_subtrees.py) | DFS / Hashing Subtrees |
| [24_Check_Mirror_N-ary_tree.py](24_Check_Mirror_N-ary_tree.py) | Recursion / Symmetry Check |
| [25_Sum_Nodes_Longest_path_from_root_leaf_node.py](25_Sum_Nodes_Longest_path_from_root_leaf_node.py) | DFS / Depth + Path Sum |
| [26_Check_graph_tree_or_not.py](26_Check_graph_tree_or_not.py) | Graph / Cycle Detection |
| [27_Find_Largest_subtree_sum_tree.py](27_Find_Largest_subtree_sum_tree.py) | Postorder DFS / Subtree Sum |
| [28_Maximum_Sum_nodes_Binary_tree_adjacent.py](28_Maximum_Sum_nodes_Binary_tree_adjacent.py) | Tree DP / Include-Exclude |
| [29_Print_all_K_Sum_paths_Binary_tree.py](29_Print_all_K_Sum_paths_Binary_tree.py) | DFS / Backtracking |
| [30_Find_LCA_Binary_tree.py](30_Find_LCA_Binary_tree.py) | DFS / Recursion |
| [31_Find_distance_between_nodes_Binary_tree.py](31_Find_distance_between_nodes_Binary_tree.py) | LCA + Depth Count |
| [32_Kth_Ancestor_node_Binary_tree.py](32_Kth_Ancestor_node_Binary_tree.py) | DFS / Backtracking |
| [33_Find_all_Duplicate_subtrees_Binary_tree.py](33_Find_all_Duplicate_subtrees_Binary_tree.py) | DFS / Subtree Serialization |
| [34_Tree_Isomorphism_Problem.py](34_Tree_Isomorphism_Problem.py) | Recursion / Tree Comparison |


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
