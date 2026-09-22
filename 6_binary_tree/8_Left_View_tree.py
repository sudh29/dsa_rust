class Node:
    def _init_(self,val):
        self.data = val
        self.left = None
        self.right = None
'''

#Function to return a list containing elements of left view of the binary tree.
def LeftView(root):
    # res = []
    # LeftViewRes(root,0,res)
    # return res
    q=[root]
    res=[]
    while q:
        n = len(q)
        for i in range(1,n+1):
            temp = q.pop(0)
            if  temp != None:
                if i == 1:
                    res.append(temp.data)
                if temp.left!=None:
                    q.append(temp.left)
                if temp.right!=None:
                    q.append(temp.right)
    return res

def LeftViewRes(root,level,res):
    if root is None:
        return
    if level>=len(res):
        res.append(root.data)
    LeftViewRes(root.left,level+1,res)
    LeftViewRes(root.right,level+1,res)
