class SortedStack:
    def __init__(self):
        self.s = []

    def sort(self):
        # Using a priority queue (max-heap) to sort the stack
        q = []

        # Move all elements from the stack to the priority queue
        while self.s:
            q.append(self.s.pop())

        # Sort the queue (which can also be viewed as a max-heap)
        q.sort(reverse=True)  # Sorting in descending order

        # Print elements in sorted order
        for item in q:
            print(item, end=" ")

        # Optionally, if you want to keep the sorted elements back in the stack:
        for item in q:
            self.s.append(item)  # Push back to stack if needed


# Example usage:
sorted_stack = SortedStack()
sorted_stack.s = [34, 3, 31, 98, 92]  # Example stack input
sorted_stack.sort()  # Output: 98 92 34 31 3
