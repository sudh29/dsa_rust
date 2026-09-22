def merge(r1, r2):
    dummy = Node(0)
    curr = dummy
    while r1 and r2:
        if r1.data < r2.data:
            curr.bottom = r1
            curr = curr.bottom
            r1 = r1.bottom
        else:
            curr.bottom = r2
            curr = curr.bottom
            r2 = r2.bottom
    if r1:
        curr.bottom = r1
    if r2:
        curr.bottom = r2
    return dummy.bottom


def flatten(root):
    # temp=[]
    # curr=root
    # while curr:
    #     temp.append(curr.data)
    #     b=curr.bottom
    #     while b:
    #         temp.append(b.data)
    #         b=b.bottom
    #     curr=curr.next
    # temp.sort()
    # res=root
    # prev=None
    # for i in temp:
    #     if res:
    #         res.data=i
    #         prev=res
    #         res=res.bottom
    #     else:
    #         temp=Node(i)
    #         prev.bottom=temp
    #         prev=prev.bottom
    # return root
    r1 = root
    r2 = root.next
    while r2:
        r1 = merge(r1, r2)
        r2 = r2.next
    return r1
