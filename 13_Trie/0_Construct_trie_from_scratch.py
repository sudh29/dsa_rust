"""
class TrieNode:

    def __init__(self):
        self.children = [None]*26

        # isEndOfWord is True if node represent the end of the word
        self.isEndOfWord = False
"""


class Solution:
    # Function to insert string into TRIE.
    def insert(self, root, key):
        currentNode = root
        for char in key:
            index = ord(char) - ord("a")
            if not currentNode.children[index]:
                currentNode.children[index] = TrieNode()
            currentNode = currentNode.children[index]
        currentNode.isEndOfWord = True

    # Function to search for a string in TRIE.
    def search(self, root, key):
        currentNode = root
        for char in key:
            index = ord(char) - ord("a")
            if not currentNode.children[index]:
                return False
            currentNode = currentNode.children[index]
        return currentNode.isEndOfWord


# {
# Driver Code Starts
# Initial Template for Python 3


# contributed by RavinderSinghPB
class TrieNode:
    def __init__(self):
        self.children = [None] * 26

        # isEndOfWord is True if node represent the end of the word
        self.isEndOfWord = False


class Trie:
    # Trie data structure class
    def __init__(self):
        self.root = TrieNode()


if __name__ == "__main__":
    t = int(input())
    for tcs in range(t):
        n = int(input())
        arr = input().strip().split()
        strs = input()

        t = Trie()
        ob = Solution()

        for s in arr:
            ob.insert(t.root, s)

        if ob.search(t.root, strs):
            print(1)
        else:
            print(0)
# } Driver Code Ends
