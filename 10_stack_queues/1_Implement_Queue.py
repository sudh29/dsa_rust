class Queue:
    def __init__(self, capacity):
        self.capacity = capacity
        self.front = 0
        self.size = 0
        self.rear = capacity - 1
        self.array = [0] * capacity  # Initialize the array with zeros

    def is_full(self):
        return self.size == self.capacity

    def is_empty(self):
        return self.size == 0

    def enqueue(self, item):
        if self.is_full():
            print("Queue is full. Cannot enqueue.")
            return
        self.rear = (self.rear + 1) % self.capacity
        self.array[self.rear] = item
        self.size += 1
        print(f"{item} enqueued to queue")

    def dequeue(self):
        if self.is_empty():
            print("Queue is empty. Cannot dequeue.")
            return None
        item = self.array[self.front]
        self.front = (self.front + 1) % self.capacity
        self.size -= 1
        return item

    def get_front(self):
        if self.is_empty():
            print("Queue is empty.")
            return None
        return self.array[self.front]

    def get_rear(self):
        if self.is_empty():
            print("Queue is empty.")
            return None
        return self.array[self.rear]


# Example usage:
queue_capacity = 5
queue = Queue(queue_capacity)

queue.enqueue(10)
queue.enqueue(20)
queue.enqueue(30)
print(queue.dequeue())  # Output: 10
print(queue.get_front())  # Output: 20
print(queue.get_rear())  # Output: 30
