class BST:
    # Function to search a node in BST.
    def search(self, node, x):
        # code here
        # if node is None:
        #     return 0
        # if node.data == x: return 1
        # elif node.data < x:
        #     return self.search(node.right,x)
        # return self.search(node.left,x)

        if root is None:
            return 0
        q = [root]
        while q:
            curr = q.pop(0)
            if curr is not None:
                if curr.data == x:
                    return 1
                elif curr.data < x:
                    q.append(curr.right)
                else:
                    q.append(curr.left)
        return 0
