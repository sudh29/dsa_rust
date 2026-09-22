# User function Template for python3
class TrieNode:
    def __init__(self):
        self.children = {}
        self.words = []


class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word, original_word):
        current = self.root
        for char in word:
            if char not in current.children:
                current.children[char] = TrieNode()
            current = current.children[char]
        current.words.append(original_word)

    def collect_anagrams(self):
        result = []
        self._collect_anagrams(self.root, result)
        return result

    def _collect_anagrams(self, node, result):
        if node.words:
            result.append((node.words))
        for child in node.children.values():
            self._collect_anagrams(child, result)


class Solution:
    def Anagrams(self, words, n):
        # anagram_groups = dict()
        # for word in words:
        #     sorted_word = ''.join(sorted(word))
        #     if sorted_word not in anagram_groups.keys():
        #         anagram_groups[sorted_word] = [word]
        #     else:
        #         anagram_groups[sorted_word].append(word)
        # return list(anagram_groups.values())

        trie = Trie()
        for word in words:
            sorted_word = "".join(sorted(word))
            trie.insert(sorted_word, word)
        anagram_groups = trie.collect_anagrams()
        return anagram_groups


# {
# Driver Code Starts
# Initial Template for Python 3

# contributed by RavinderSinghPB
if __name__ == "__main__":
    t = int(input())
    for tcs in range(t):
        n = int(input())
        words = input().split()

        ob = Solution()
        ans = ob.Anagrams(words, n)

        for grp in sorted(ans):
            for word in grp:
                print(word, end=" ")
            print()

# } Driver Code Ends
