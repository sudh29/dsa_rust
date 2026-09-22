def inorder(node, res):
    if node is None:
        return
    inorder(node.left, res)
    res.append(node.data)
    inorder(node.right, res)


def findMedian(root):
    r1 = []
    inorder(root, r1)
    n = len(r1)
    mid = n // 2
    if n % 2 != 0:
        return r1[mid]
    val = (r1[mid] + r1[mid - 1]) / 2
    return int(val) if int(val) == val else val
