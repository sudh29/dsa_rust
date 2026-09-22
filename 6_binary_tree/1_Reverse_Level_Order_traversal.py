def reverseLevelOrder(root):
    # code here
    # Code here
    res = []
    queue = [root]
    while queue:
        value = queue.pop(0)
        res.append(value.data)
        if value.right:
            queue.append(value.right)
        if value.left:
            queue.append(value.left)
    return res[::-1]
