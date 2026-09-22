class TrieNode:
    def __init__(self):
        self.children = {}
        self.contacts = set()


class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word):
        node = self.root
        for char in word:
            if char not in node.children:
                node.children[char] = TrieNode()
            node = node.children[char]
            node.contacts.add(word)

    def search(self, prefix):
        node = self.root
        for char in prefix:
            if char not in node.children:
                return []
            node = node.children[char]
        return sorted(node.contacts)


class Solution:
    def displayContacts(self, n, contact, s):
        trie = Trie()
        for con in contact:
            trie.insert(con)

        results = []
        for i in range(1, len(s) + 1):
            prefix = s[:i]
            matches = trie.search(prefix)
            if matches:
                results.append(matches)
            else:
                results.append(["0"])
        return results


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        n = int(input())
        contact = input().split()
        s = input()

        ob = Solution()
        ans = ob.displayContacts(n, contact, s)
        for i in range(len(s)):
            for val in ans[i]:
                print(val, end=" ")
            print()
# } Driver Code Ends
