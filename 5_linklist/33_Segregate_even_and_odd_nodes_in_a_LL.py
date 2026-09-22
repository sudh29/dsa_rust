class Solution:
    def divide(self, N, head):
        # curr=head
        # temp=[]
        # while curr:
        #     temp.append(curr.data)
        #     curr=curr.next
        # temp1=[]
        # temp2=[]
        # for i in temp:
        #     if i%2==0:
        #         temp1.append(i)
        #     else:
        #         temp2.append(i)
        # temp=temp1+temp2
        # curr=head
        # for i in temp:
        #     curr.data=i
        #     curr=curr.next
        # return head

        end = head
        prev = None
        curr = head
        while end.next != None:
            end = end.next
        new_end = end
        while curr.data % 2 != 0 and curr != end:
            new_end.next = curr
            curr = curr.next
            new_end.next.next = None
            new_end = new_end.next
        if curr.data % 2 == 0:
            head = curr
            while curr != end:
                if curr.data % 2 == 0:
                    prev = curr
                    curr = curr.next
                else:
                    prev.next = curr.next
                    curr.next = None
                    new_end.next = curr
                    new_end = curr
                    curr = prev.next
        else:
            prev = curr
        if new_end != end and end.data % 2 != 0:
            prev.next = end.next
            end.next = None
            new_end.next = end
        return head
