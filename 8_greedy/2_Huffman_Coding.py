# User function Template for python3
import heapq


class Node:
    def __init__(self, char, freq):
        self.char = char
        self.freq = freq
        self.left = None
        self.right = None
        self.huff = ""

    def __lt__(self, nxt):
        return self.freq < nxt.freq


def getList(ans, node, val=""):
    newval = val + str(node.huff)
    if node.left:
        getList(ans, node.left, newval)
    if node.right:
        getList(ans, node.right, newval)
    if not node.left and not node.right:
        ans.append(newval)


class Solution:
    def huffmanCodes(self, S, f, N):
        nodes = []
        for i in range(N):
            heapq.heappush(nodes, Node(S[i], f[i]))

        while len(nodes) > 1:
            left = heapq.heappop(nodes)
            right = heapq.heappop(nodes)
            left.huff = "0"
            right.huff = "1"
            merged_node = Node(None, left.freq + right.freq)
            merged_node.left = left
            merged_node.right = right
            heapq.heappush(nodes, merged_node)

        ans = []
        getList(ans, nodes[0])
        return ans


# {
# Driver Code Starts
# Initial Template for Python 3


if __name__ == "__main__":
    t = int(input())
    for i in range(t):
        S = input()
        N = len(S)
        f = [int(x) for x in input().split()]
        ob = Solution()
        ans = ob.huffmanCodes(S, f, N)
        for i in ans:
            print(i, end=" ")
        print()
# } Driver Code Ends
