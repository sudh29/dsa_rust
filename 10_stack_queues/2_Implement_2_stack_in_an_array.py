class TwoStacks:
    def __init__(self, size):
        self.size = size
        self.arr = [0] * size  # Initialize the array with zeros
        self.top1 = -1  # Stack 1 starts from the beginning
        self.top2 = size  # Stack 2 starts from the end

    def push1(self, x):
        if self.top2 - self.top1 > 1:  # Check for space in stack 1
            self.top1 += 1
            self.arr[self.top1] = x
        else:
            print("Stack 1 is full")

    def push2(self, x):
        if self.top2 - self.top1 > 1:  # Check for space in stack 2
            self.top2 -= 1
            self.arr[self.top2] = x
        else:
            print("Stack 2 is full")

    def pop1(self):
        if self.top1 == -1:  # Check if stack 1 is empty
            return -1
        popped_value = self.arr[self.top1]
        self.top1 -= 1
        return popped_value

    def pop2(self):
        if self.top2 == self.size:  # Check if stack 2 is empty
            return -1
        popped_value = self.arr[self.top2]
        self.top2 += 1
        return popped_value


# Example usage:
stack_size = 5
two_stacks = TwoStacks(stack_size)

two_stacks.push1(10)
two_stacks.push1(20)
print(two_stacks.pop1())  # Output: 20

two_stacks.push2(30)
two_stacks.push2(40)
print(two_stacks.pop2())  # Output: 40
