class Node:
    def __init__(self, x):
        self.data = x
        self.next = None
        self.prev = None

class DoublyLinkedList:
    def __init__(self):
        self.dummy = Node(-1)
        self.head = self.dummy
        self.mid = self.dummy
        self.count = 0

    def push(self, data):
        curr = Node(data)
        curr.prev = None
        curr.next = self.head
        self.count += 1

        if self.head != self.dummy:  # If head is not the dummy node
            self.head.prev = curr

        self.head = curr

        if self.count == 1:
            self.mid = curr
        elif self.count % 2 == 0:
            self.mid = self.mid.prev

    def pop(self):
        if self.count == 0:
            print("Empty")
            return -1

        temp = self.head
        x = self.head.data
        self.head = self.head.next

        if self.head != self.dummy:  # If there's a node left after popping
            self.head.prev = None

        self.count -= 1

        if self.count % 2 == 1:
            self.mid = self.mid.next

        del temp  # Python's garbage collector will handle this
        return x

    def middle(self):
        if self.count == 0:
            print("Empty")
            return -1
        return self.mid.data

    def print_data(self):
        curr = self.head
        if self.count == 0:
