# User function Template for python3
import heapq


class Solution:
    def rearrangeString(self, str):
        char_count = {}
        for char in str:
            char_count[char] = char_count.get(char, 0) + 1

        heap = [(-count, char) for char, count in char_count.items()]
        heapq.heapify(heap)
        res = []
        while len(heap) > 1:
            count1, char1 = heapq.heappop(heap)
            count2, char2 = heapq.heappop(heap)
            res.extend([char1, char2])
            if count1 < -1:
                heapq.heappush(heap, (count1 + 1, char1))
            if count2 < -1:
                heapq.heappush(heap, (count2 + 1, char2))
        if heap:
            count1, char1 = heapq.heappop(heap)
            if res and char1 == res[-1]:
                return "-1"
            res.append(char1)
        return "".join(res)


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
