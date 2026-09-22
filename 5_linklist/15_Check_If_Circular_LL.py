def isCircular(head):
    if head == None:
        return True
    curr = head
    while curr:
        if curr.next == head:
            return True
        curr = curr.next
    return False
