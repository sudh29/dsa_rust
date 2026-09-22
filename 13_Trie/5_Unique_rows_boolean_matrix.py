from typing import List


class TrieNode:
    def __init__(self):
        self.children = {}
        self.is_end_of_row = False


class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, row: List[int]) -> bool:
        current_node = self.root
        for num in row:
            if num not in current_node.children:
                current_node.children[num] = TrieNode()
            current_node = current_node.children[num]

        if current_node.is_end_of_row:
            return False
        else:
            current_node.is_end_of_row = True
            return True


class Solution:
    def uniqueRow(self, row: int, col: int, M: List[List[int]]) -> List[List[int]]:
        # res = []
        # unique_rows = set()
        # for i in range(row):
        #     row_tuple = tuple(M[i])
        #     if row_tuple not in unique_rows:
        #         unique_rows.add(row_tuple)
        #         res.append(M[i])
        # return res

        trie = Trie()
        res = []
        for i in range(row):
            if trie.insert(M[i]):
                res.append(M[i])
        return res


# {
# Driver Code Starts
# Initial Template for Python 3


def main():
    testcase = int(input())
    while testcase:
        s = input().split()
        row = int(s[0])
        col = int(s[1])
        matrix = [[None for _ in range(col)] for _ in range(row)]
        s = input().split()
        for i in range(row):
            for j in range(col):
                matrix[i][j] = int(s[i * col + j])

        ob = Solution()
        a = ob.uniqueRow(row, col, matrix)

        for row in a:
            for value in row:
                print(value, end=" ")
            print("$", end="")
        print()
        testcase -= 1


if __name__ == "__main__":
    main()
# } Driver Code Ends
