class Solution:
    def middleNode(self, head: Optional[ListNode]) -> Optional[ListNode]:
        #         len1=0
        #         curr=head
        #         while curr:
        #             len1+=1
        #             curr=curr.next

        #         n=len1//2
        #         curr=head
        #         for i in range(n):
        #             curr=curr.next
        #         return curr
        mid = head
        while head and head.next:
            mid = mid.next
            head = head.next.next
        return mid
