# User function Template for python3


class Solution:
    def printSequence(self, S):
        # code here
        # arr = ["2", "22", "222",
        #       "3", "33", "333",
        #       "4", "44", "444",
        #       "5", "55", "555",
        #       "6", "66", "666",
        #       "7", "77", "777", "7777",
        #       "8", "88", "888",
        #       "9", "99", "999", "9999"]

        # n = len(S)
        # output = ""
        # for i in range(n):
        #     if S[i] == ' ':
        #         output = output + "0"
        #     else:
        #         position = ord(S[i]) - ord('A')
        #         output = output + arr[position]
        # return output

        mapping = {
            "A": "2",
            "B": "22",
            "C": "222",
            "D": "3",
            "E": "33",
            "F": "333",
            "G": "4",
            "H": "44",
            "I": "444",
            "J": "5",
            "K": "55",
            "L": "555",
            "M": "6",
            "N": "66",
            "O": "666",
            "P": "7",
            "Q": "77",
            "R": "777",
            "S": "7777",
            "T": "8",
            "U": "88",
            "V": "888",
            "W": "9",
            "X": "99",
            "Y": "999",
            "Z": "9999",
            " ": "0",
        }

        output = "".join(mapping[char] for char in S)
        return output


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())

    for _ in range(t):
        inputStr = input()

        solObj = Solution()

        print(solObj.printSequence(inputStr))
# } Driver Code Ends
