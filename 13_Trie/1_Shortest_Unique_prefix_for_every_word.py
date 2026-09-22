class TrieNode:
    def __init__(self):
        self.children = {}
        self.count = 0


class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word):
        current = self.root
        for char in word:
            if char not in current.children:
                current.children[char] = TrieNode()
            current = current.children[char]
            current.count += 1

    def find_unique_prefix(self, word):
        current = self.root
        prefix = ""
        for char in word:
            prefix += char
            current = current.children[char]
            if current.count == 1:
                break
        return prefix


class Solution:
    def findPrefixes(self, arr, N):
        trie = Trie()
        for word in arr:
            trie.insert(word)

        unique_prefixes = []
        for word in arr:
            unique_prefixes.append(trie.find_unique_prefix(word))
        return unique_prefixes


# {
# Driver Code Starts
# Initial Template for Python 3

import sys

sys.setrecursionlimit(10**6)
if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        N = int(input())
        arr = list(map(str, input().split()))

        ob = Solution()
        res = ob.findPrefixes(arr, N)
        for i in res:
            print(i, end=" ")
        print()
