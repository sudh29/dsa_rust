def getNthFromLast(head,n):
    #code here
     '''temp=head
    c=0
    while temp!=None:
        temp=temp.next
        c+=1
    if n>c:
        return -1
    n_new=c-n
    temp=head
    for i in range(n_new):
        temp=temp.next
    return temp.data'''

    slow=head
    fast=head
    for i in range(1,n):
        if fast.next != None:
            fast=fast.next
        else:
            return -1
    #print(fast.data)
    while fast and fast.next:
        fast=fast.next
        slow=slow.next
    return slow.data
