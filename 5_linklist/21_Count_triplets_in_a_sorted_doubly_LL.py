def countTriplets(head, x):
    count = 0
    ptr1 = head
    ptr3 = head
    while ptr3.next:
        ptr3 = ptr3.next
    while ptr1.next:
        new_x = x - ptr1.data
        firstnode = ptr1.next
        lastnode = ptr3
        while firstnode != lastnode:
            temp_sum = firstnode.data + lastnode.data
            if temp_sum > new_x:
                lastnode = lastnode.prev
            elif temp_sum < new_x:
                firstnode = firstnode.next
            else:
                count += 1
                firstnode = firstnode.next
                if firstnode == lastnode:
                    break
                lastnode = lastnode.prev
        ptr1 = ptr1.next
    return count
