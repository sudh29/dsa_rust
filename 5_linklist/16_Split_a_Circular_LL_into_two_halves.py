class Solution:
    def splitList(self, head, head1, head2):
        head1 = head
        first = head
        second = head.next
        while second != head and second.next != head:
            first = first.next
            second = second.next.next
        head2 = first.next
        first.next = head
        temp = head2
        while temp.next != head:
            temp = temp.next
        temp.next = head2
        return head1, head2
