    def bottomView(self, root):
        # code here
        if root is None:
            return
        l=0
        r=0
        q=[(root,0)]
        m = dict()
        while q:
            curr, hd = q.pop(0)
            m[hd] = curr.data
            l = min(l,hd)
            r = max(r,hd)
            if curr.left: q.append((curr.left,hd-1))
            if curr.right: q.append((curr.right,hd+1))

        res=[]
        for i in range(l,r+1):
            res.append(m[i])
        return res
