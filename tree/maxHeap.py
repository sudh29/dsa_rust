# max Heap

from heapq import heapify, heappop, heappush

heap = []
heapify(heap)

heappush(heap, -1 * 10)
heappush(heap, -1 * 30)
heappush(heap, -1 * 20)
heappush(heap, -1 * 400)

heap = [-1 * i for i in heap]
print(heap)
heap = [-1 * i for i in heap]

heappop(heap)

for i in heap:
    print(-1 * i, end=" ")

print()
