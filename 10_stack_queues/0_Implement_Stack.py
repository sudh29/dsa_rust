class Stack:
    MAX = 1000

    def __init__(self):
        self.top = -1
        self.a = [0] * Stack.MAX  # Initialize stack with zeros

    def push(self, x):
        if self.top >= (Stack.MAX - 1):
            print("Overflow")
            return False
        else:
            self.top += 1
            self.a[self.top] = x
            print(f"{x} pushed into stack")
            return True

    def pop(self):
        if self.top < 0:
            print("Underflow")
            return 0
        else:
            x = self.a[self.top]
            self.top -= 1
            return x

    def peek(self):
        if self.top < 0:
            print("Empty")
            return 0
        else:
            return self.a[self.top]

    def is_empty(self):
        return self.top < 0


# Example usage:
stack = Stack()
stack.push(10)
stack.push(20)
print(stack.peek())  # Output: 20
print(stack.pop())  # Output: 20
print(stack.is_empty())  # Output: False
