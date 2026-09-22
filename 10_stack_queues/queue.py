# queue implementation

from collections import deque

q = deque()

q.append("a")
q.append("b")
q.append("c")

print("Queue")
print(q)
q.popleft()
q.popleft()
print("After deque")
print(q)
