class Solution:
    # Function to remove duplicates from unsorted linked list.
    def removeDuplicates(self, head):
        # code here
        # return head after editing list
        curr = head
        elements = set()
        elements.add(curr.data)
        while curr.next:
            if curr.next.data in elements:
                curr.next = curr.next.next
            else:
                elements.add(curr.next.data)
                curr = curr.next
        return head
