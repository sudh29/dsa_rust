class Solution:
    def reverse(self,head, k):
       '''curr=head
        temp=[]
        while curr:
            temp.append(curr.data)
            curr=curr.next
        for i in range(0,len(temp),k):
            if i+k<=len(temp):
                val=temp[i:i+k]
                temp[i:i+k]=val[::-1]
            else:
                val=temp[i:]
                temp[i:]=val[::-1]
        curr=head
        for i in range(len(temp)):
            curr.data=temp[i]
            curr=curr.next
        return head'''

        curr=head
        prev=None
        c=0
        while curr and c<k:
            next=curr.next
            curr.next=prev
            prev=curr
            curr=next
            c+=1
        if next:
            head.next=self.reverse(next,k)
        return prev
