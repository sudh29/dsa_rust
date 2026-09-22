class Solution:
    # Function to check if the linked list has a loop.
    def detectLoop(self, head):
        # code here
        first = head
        second = head
        while second and second.next:
            first = first.next
            second = second.next.next
            if first == second:
                return True
        return False
