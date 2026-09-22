# User function Template for python3
from collections import Counter
import heapq


class Solution:
    def rearrangeString(self, S):
        char_count = Counter(S)
        max_heap = [(-freq, char) for char, freq in char_count.items()]
        heapq.heapify(max_heap)
        prev_char, prev_freq = None, 0
        result = []
        while max_heap:
            freq, char = heapq.heappop(max_heap)
            result.append(char)
            if prev_char and prev_freq < 0:
                heapq.heappush(max_heap, (prev_freq, prev_char))
            prev_char = char
            prev_freq = freq + 1
        rearranged_str = "".join(result)
        return rearranged_str if len(rearranged_str) == len(S) else "-1"


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        str1 = input()
        solObj = Solution()
        str2 = solObj.rearrangeString(str1)
        if str2 == "-1":
            print(0)
        elif sorted(str1) != sorted(str2):
            print(0)
        else:
            for i in range(len(str2) - 1):
                if str2[i] == str2[i + 1]:
                    print(0)
                    break
            else:
                print(1)

# } Driver Code Ends
