class StackOperations:
    def push(self, s, a):
        s.append(a)

    def isFull(self, s, n):
        return len(s) == n

    def isEmpty(self, s):
        return len(s) == 0

    def pop(self, s):
        if len(s) > 0:
            a = s[-1]  # Get the top element
            s.pop()  # Remove the top element
            return a
        else:
            return -1

    def getMin(self, s):
        if self.isEmpty(s):
            return -1  # Return -1 if stack is empty

        min_val = float("inf")  # Initialize min_val to infinity
        while not self.isEmpty(s):
            a = s.pop()  # Pop the top element
            min_val = min(min_val, a)  # Update min_val
        return min_val
