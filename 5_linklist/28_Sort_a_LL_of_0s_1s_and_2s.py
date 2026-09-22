class Solution:
    # Function to sort a linked list of 0s, 1s and 2s.
    def segregate(self, head):
        c0 = 0
        c1 = 0
        c2 = 0
        curr = head
        while curr:
            if curr.data == 0:
                c0 += 1
            elif curr.data == 1:
                c1 += 1
            elif curr.data == 2:
                c2 += 1
            curr = curr.next

        curr = head
        while curr:
            if c0:
                curr.data = 0
                c0 -= 1
            elif c1:
                curr.data = 1
                c1 -= 1
            elif c2:
                curr.data = 2
                c2 -= 1
            curr = curr.next
        return head
