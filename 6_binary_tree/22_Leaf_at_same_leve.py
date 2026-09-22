class Solution:
    # Your task is to complete this function
    # function should return True/False or 1/0
    def check_level(self, root, level, leaf_level):
        if not root:
            return True

        if not root.left and not root.right:
            if leaf_level[0] is None:
                leaf_level[0] = level
            else:
                if leaf_level[0] != level:
                    return False

        left_result = self.check_level(root.left, level + 1, leaf_level)
        right_result = self.check_level(root.right, level + 1, leaf_level)

        return left_result and right_result

    def check(self, root):
        # if root is None:
        #     return True
        # q=[(root,0)]
        # first_level = None
        # while q:
        #     curr, level = q.pop(0)
        #     if not curr.left and not curr.right:
        #         if first_level is None:
        #             first_level = level
        #         else:
        #             if first_level!=level:
        #                 return False
        #     if curr.left: q.append((curr.left, level + 1))
        #     if curr.right: q.append((curr.right, level + 1))
        # return True

        leaf_level = [None]
        return self.check_level(root, 0, leaf_level)
