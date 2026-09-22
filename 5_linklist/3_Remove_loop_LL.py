class Solution:
    # Function to remove a loop in the linked list.
    def removeLoop(self, head):
        # code here
        # remove the loop without losing any nodes
        first = head
        second = head
        node = None
        while first and second and second.next:
            first = first.next
            second = second.next.next
            if first == second:
                node = first
                break
        if node != None:
            first = node
            second = node
            k = 1
            while first.next != second:
                first = first.next
                k += 1
            first = head
            second = head
            for i in range(k):
                second = second.next
            while first != second:
                first = first.next
                second = second.next
            while second.next != first:
                second = second.next
            second.next = None
