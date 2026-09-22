class Solution:
    # Function to check if brackets are balanced or not.
    def ispar(self, x):
        temp = {"(": ")", "[": "]", "{": "}"}
        stack = []
        for i in x:
            if i in temp.keys():
                stack.append(i)
            elif len(stack) == 0:
                return False
            else:
                val = stack.pop()
                if temp[val] == i:
                    pass
                else:
                    return False
        return True if len(stack) == 0 else False
