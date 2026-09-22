# User function Template for python3
class TrieNode:
    def __init__(self):
        self.children = {}
        self.isEndOfWord = False


class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word):
        current = self.root
        for char in word:
            if char not in current.children:
                current.children[char] = TrieNode()
            current = current.children[char]
        current.isEndOfWord = True

    def search(self, word):
        current = self.root
        for char in word:
            if char not in current.children:
                return False
            current = current.children[char]
        return current.isEndOfWord


class Solution:
    def wordBreak(self, n, s, dictionary):
        # word_set = set(dictionary)
        # memo = {}

        # def can_break(start):
        #     if start == len(s):
        #         return True
        #     if start in memo:
        #         return memo[start]
        #     for end in range(start + 1, len(s) + 1):
        #         if s[start:end] in word_set and can_break(end):
        #             memo[start] = True
        #             return True
        #     memo[start] = False
        #     return False

        # return 1 if can_break(0) else 0

        trie = Trie()
        for word in dictionary:
            trie.insert(word)

        dp = [False] * (len(s) + 1)
        dp[0] = True

        for i in range(1, len(s) + 1):
            for j in range(i):
                if dp[j] and trie.search(s[j:i]):
                    dp[i] = True
                    break
        return 1 if dp[-1] else 0


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    test_case = int(input())

    for _ in range(test_case):
        n = int(input())
        dictionary = [word for word in input().strip().split()]
        s = input().strip()
        ob = Solution()
        res = ob.wordBreak(n, s, dictionary)
        if res:
            print(1)
        else:
            print(0)
# } Driver Code Ends
