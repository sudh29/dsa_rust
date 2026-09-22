# min Heap

from heapq import heapify, heappop, heappush

heap = []
heapify(heap)

heappush(heap, 10)
heappush(heap, 30)
heappush(heap, 20)
heappush(heap, 400)


print(heap)

heappop(heap)

print(heap)
