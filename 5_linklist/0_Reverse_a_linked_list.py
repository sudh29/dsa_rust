class Solution:
    # Function to reverse a linked list.
    def reverseList(self, head):
        # Code here
        curr = head
        prev = None
        while curr:
            next = curr.next
            curr.next = prev
            prev = curr
            curr = next
        head = prev
        return head


#     recursion
#     def reverse(node):
#     if (node == None):
#         return node

#     if (node.next == None):
#         return node

#     node1 = reverse(node.next)
#     node.next.next = node
#     node.next = None
#     return node1
