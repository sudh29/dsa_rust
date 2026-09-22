from typing import List


class Solution:
    def sentences(self, L: List[List[str]]) -> List[List[str]]:
        def solve(index):
            if index == len(L):
                return [[]]
            partial_sentences = solve(index + 1)
            full_sentences = []
            for word in L[index]:
                for sentence in partial_sentences:
                    full_sentences.append([word] + sentence)
            return full_sentences

        # result = solve(0)
        # return [w for w in result]

        result = [""]
        for words in L:
            new_result = []
            for sentence in result:
                for word in words:
                    new_result.append(sentence + " " + word if sentence else word)
            result = new_result
        return [[w] for w in result]


# {
# Driver Code Starts
class IntArray:
    def __init__(self) -> None:
        pass

    def Input(self, n):
        arr = [int(i) for i in input().strip().split()]  # array input
        return arr

    def Print(self, arr):
        for i in arr:
            print(i, end=" ")
        print()


class StringMatrix:
    def __init__(self) -> None:
        pass

    def Input(self, n, m):
        matrix = []
        # matrix input
        for _ in range(n):
            matrix.append([str(i) for i in input().strip().split()])
        return matrix

    def Print(self, arr):
        for i in arr:
            for j in i:
                print(j, end=" ")
            print()


if __name__ == "__main__":
    a = IntArray().Input(2)

    list = StringMatrix().Input(a[0], a[1])

    obj = Solution()
    res = obj.sentences(list)

    StringMatrix().Print(res)


# } Driver Code Ends
