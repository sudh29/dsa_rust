class Solution:
    # Function to return list containing elements of right view of binary tree.
    def rightView(self, root):
        # res = []
        # self.rightViewCal(root,0,res)
        # return res
        q = [root]
        res = []
        while q:
            n = len(q)
            for i in range(1, n + 1):
                temp = q.pop(0)
                if temp != None:
                    if i == 1:
                        res.append(temp.data)
                    if temp.right != None:
                        q.append(temp.right)
                    if temp.left != None:
                        q.append(temp.left)
        return res

    def rightViewCal(self, root, level, res):
        if root is None:
            return
        if level >= len(res):
            res.append(root.data)
        self.rightViewCal(root.right, level + 1, res)
        self.rightViewCal(root.left, level + 1, res)
