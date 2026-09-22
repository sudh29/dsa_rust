class Solution:
    # Function to add two numbers represented by linked list.
    def addTwoLists(self, first, second):
        # code here
        # return head of sum list

        curr = first
        s1 = ""
        while curr:
            s1 += str(curr.data)
            curr = curr.next
        curr = second
        s2 = ""
        while curr:
            s2 += str(curr.data)
            curr = curr.next
        m = len(s1)
        n = len(s2)
        res = str(int(s1) + int(s2))
        curr = first if m > n else second
        prev = None
        for i in res:
            if curr:
                curr.data = int(i)
                prev = curr
                curr = curr.next
            else:
                curr = Node(int(i))
                prev.next = curr
                prev = curr
                curr = curr.next
        return first if m > n else second
