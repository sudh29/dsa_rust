# Function to find intersection point in Y shaped Linked Lists.
def intersetPoint(head1, head2):
    # code here
    len1 = 0
    curr = head1
    while curr:
        len1 += 1
        # print(curr.data)
        curr = curr.next
    len2 = 0
    curr = head2
    while curr:
        len2 += 1
        # print(" ",curr.data)
        curr = curr.next

    curr1 = head1
    curr2 = head2
    if len1 > len2:
        for i in range(len1 - len2):
            curr1 = curr1.next
    elif len2 > len1:
        for i in range(len2 - len1):
            curr2 = curr2.next
    while curr1 != curr2:
        curr1 = curr1.next
        curr2 = curr2.next
    return curr1.data
