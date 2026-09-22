# User function Template for python3


class Solution:
    def Anagrams(self, words, n):
        """
        words: list of word
        n:      no of words
        return : list of group of anagram {list will be sorted in driver code (not word in grp)}
        """
        anagram_groups = dict()
        for word in words:
            sorted_word = "".join(sorted(word))
            if sorted_word not in anagram_groups.keys():
                anagram_groups[sorted_word] = [word]
            else:
                anagram_groups[sorted_word].append(word)
        return list(anagram_groups.values())


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
