class Solution:
    def isPalindrome(self, head):
        # code here
        temp = []
        curr = head
        while curr:
            temp.append(curr.data)
            curr = curr.next
        for i in range(int(len(temp) / 2)):
            if temp[i] != temp[len(temp) - 1 - i]:
                return False
        return True
