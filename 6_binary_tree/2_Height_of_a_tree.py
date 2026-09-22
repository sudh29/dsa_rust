class Solution:
    # Function to find the height of a binary tree.
    def height(self, root):
        # code here
        # if root is None:
        #     return 0
        # else:
        #     ldepth = self.height(root.left)
        #     rdepth = self.height(root.right)
        #     if ldepth>rdepth: return ldepth+1
        #     else: return rdepth+1
        h = 0
        queue = [root]
        while queue:
            nodecnt = len(queue)
            while nodecnt:
                value = queue.pop(0)

                if value.left:
                    queue.append(value.left)
                if value.right:
                    queue.append(value.right)
                nodecnt -= 1
            h += 1
        return h
