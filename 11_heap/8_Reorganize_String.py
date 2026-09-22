import heapq


class Solution:
    def reorganizeString(self, s: str) -> str:
        char_count = {}
        for char in s:
            char_count[char] = char_count.get(char, 0) + 1

        heap = [(-count, char) for char, count in char_count.items()]
        heapq.heapify(heap)
        result = []
        while len(heap) >= 2:
            count1, char1 = heapq.heappop(heap)
            count2, char2 = heapq.heappop(heap)
            result.extend([char1, char2])
            if count1 < -1:
                heapq.heappush(heap, (count1 + 1, char1))
            if count2 < -1:
                heapq.heappush(heap, (count2 + 1, char2))

        if heap:
            count, char = heapq.heappop(heap)
            if result and result[-1] == char:
                return ""
            result.append(char)

        return "".join(result) if len(result) == len(s) else ""
