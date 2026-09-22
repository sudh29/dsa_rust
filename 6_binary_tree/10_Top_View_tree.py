class Solution:
    # Function to return a list of nodes visible from the top view
    # from left to right in Binary Tree.
    def topView(self, root):
        if root is None:
            return
        l = 0
        r = 0
        q = [(root, 0)]
        m = dict()
        while q:
            curr, hd = q.pop(0)
            if hd not in m.keys():
                m[hd] = curr.data
            if curr.left != None:
                q.append((curr.left, hd - 1))
                l = min(l, hd - 1)
            if curr.right != None:
                q.append((curr.right, hd + 1))
                r = max(r, hd + 1)
        res = []
        for i in range(l, r + 1):
            res.append(m[i])
        return res
