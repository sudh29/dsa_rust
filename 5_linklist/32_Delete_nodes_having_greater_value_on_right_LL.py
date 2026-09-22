class Solution:
    def compute(self, head):
        """curr=head
        temp=[]
        while curr:
            temp.append(curr.data)
            curr=curr.next

        maxval=max(temp)
        idx=temp.index(maxval)
        res=[maxval]
        i=idx+1
        while i<len(temp):
            maxval=max(temp[i:])
            res.append(maxval)
            idx=temp.index(maxval)
            i=idx+1

        prev=Node(0)
        curr=prev
        for i in res:
            curr.next=Node(i)
            curr=curr.next
        return prev.next"""
        ####################
        # curr=head
        # prev=Node(0)
        # prev.next=head
        # ans=prev
        # while curr:
        #     temp=curr.next
        #     while temp and temp.data<=curr.data:
        #         temp=temp.next
        #     if temp:
        #         prev.next=curr.next
        #         curr=prev.next
        #     else:
        #         prev=prev.next
        #         curr=curr.next
        # return ans.next

        ####################
        if not (head.next):
            return head
        head.next = self.compute(head.next)
        return head.next if (head.data < head.next.data) else head
        ###################
        # reverse iterate reverse
