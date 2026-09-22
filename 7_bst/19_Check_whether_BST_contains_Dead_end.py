class Solution:
    def isDeadEnd(self, root):
        # def inorder(node,ans,leaf):
        #     if node is None:
        #         return
        #     ans.append(node.data)
        #     if node.left is None and node.right is None:
        #         leaf.append(node.data)
        #     inorder(node.left,ans,leaf)
        #     inorder(node.right,ans,leaf)

        # res = []
        # res2 = []
        # inorder(root,res,res2)
        # flag=False
        # for i in res2:
        #     if i==1:
        #         if i+1 in res:
        #             flag=True
        #     else:
        #         if i-1 in res and i+1 in res:
        #             flag = True
        # return flag

        def has_dead_end(root, min_val=1, max_val=float("inf")):
            if not root:
                return False
            if min_val == max_val:
                return True

            left_dead_end = has_dead_end(root.left, min_val, root.data - 1)
            right_dead_end = has_dead_end(root.right, root.data + 1, max_val)
            return left_dead_end or right_dead_end

        return has_dead_end(root)
