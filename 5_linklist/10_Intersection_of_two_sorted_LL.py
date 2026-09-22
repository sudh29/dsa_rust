def findIntersection(head1, head2):
    # return head

    curr1 = head1
    curr2 = head2
    ans = Node(-1)
    temp = ans
    while curr1 and curr2:
        val1 = curr1.data
        val2 = curr2.data
        if val1 == val2:
            temp.next = Node(val1)
            temp = temp.next
            curr1 = curr1.next
            curr2 = curr2.next
        elif val1 < val2:
            curr1 = curr1.next
        else:
            curr2 = curr2.next
    return ans.next
