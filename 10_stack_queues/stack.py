# implement stack using deque LIFO

from collections import deque

q = deque()

q.append("a")
q.append("b")
q.append("c")

print("Queue")
print(q)
q.pop()
q.pop()
print("After pop")
print(q)
