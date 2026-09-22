class Solution:
    # Function to return the diameter of a Binary Tree.
    # def height(self,root):
    #     # Code here
    #     if root is None:
    #         return 0
    #     else:
    #         ldepth = self.height(root.left)
    #         rdepth = self.height(root.right)
    #         return max(ldepth,rdepth)+1

    # def diameter(self,root):
    #     # Code here
    #     if root is None:
    #         return 0
    #     else:
    #         ldiameter = self.diameter(root.left)
    #         rdiameter = self.diameter(root.right)
    #         return max(self.height(root.left)+self.height(root.right)+1,max(ldiameter,rdiameter))

    def diameter(self, root):
        def height_and_diameter(node):
            if not node:
                return 0, 0  # height, diameter

            left_height, left_diameter = height_and_diameter(node.left)
            right_height, right_diameter = height_and_diameter(node.right)

            # Calculate height and diameter for the current node
            current_height = 1 + max(left_height, right_height)
            current_diameter = max(
                left_height + right_height + 1, max(left_diameter, right_diameter)
            )

            return current_height, current_diameter

        _, diameter = height_and_diameter(root)
        return diameter
