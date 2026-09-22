class KStacks:
    def __init__(self, k, n):
        self.k = k  # Number of stacks
        self.n = n  # Size of the array
        self.arr = [0] * n  # Array to hold stack elements
        self.top = [-1] * k  # Initialize tops of all stacks
        self.next = list(range(1, n)) + [-1]  # Next array for free nodes
        self.free = 0  # Free index to track free space

    def is_full(self):
        return self.free == -1

    def push(self, item, sn):
        if self.is_full():
            print("\nStack Overflow\n")
            return

        # Get the index of the free slot
        i = self.free
        self.free = self.next[i]  # Update free to next free index
        self.next[i] = self.top[sn]  # Link new element to previous top
        self.top[sn] = i  # Update the top of the stack
        self.arr[i] = item  # Insert the item

    def pop(self, sn):
        if self.is_empty(sn):
            print("\nStack Underflow\n")
            return float("inf")  # Return a large number

        # Get the index of the top item
        i = self.top[sn]
        self.top[sn] = self.next[i]  # Update the top of the stack
        self.next[i] = self.free  # Update next to free
        self.free = i  # Mark this index as free
        return self.arr[i]  # Return the popped item

    def is_empty(self, sn):
        return self.top[sn] == -1


# Example usage
if __name__ == "__main__":
    k = 3  # Number of stacks
    n = 10  # Total size of the array
    ks = KStacks(k, n)

    ks.push(15, 2)
    ks.push(45, 2)
    ks.push(17, 1)
    ks.push(49, 1)
    ks.push(39, 1)
    ks.push(11, 0)
    ks.push(9, 0)
    ks.push(7, 0)

    print("Popped element from stack 2 is", ks.pop(2))  # Expected output: 45
    print("Popped element from stack 1 is", ks.pop(1))  # Expected output: 39
    print("Popped element from stack 0 is", ks.pop(0))  # Expected output: 7
