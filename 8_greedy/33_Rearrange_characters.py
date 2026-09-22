# User function Template for python3


class Solution:
    def rearrangeString(self, str):
        list_char = [0 for i in range(26)]
        for i in str:
            list_char[ord(i) - ord("a")] += 1

        max_freq, letter = 0, 0
        for i in range(len(list_char)):
            if list_char[i] > max_freq:
                max_freq = list_char[i]
                letter = i

        n = len(str)
        half_len = n // 2 if n % 2 == 0 else (n + 1) // 2
        if max_freq > half_len:
            return ""

        res = [""] * len(str)
        # Fill all even places with the majority character
        idx = 0
        while list_char[letter] > 0:
            res[idx] = chr(letter + ord("a"))
            idx += 2
            list_char[letter] -= 1
        # Fill the remaining characters
        for i in range(len(list_char)):
            while list_char[i] > 0:
                if idx >= len(res):
                    idx = 1
                res[idx] = chr(i + ord("a"))
                idx += 2
                list_char[i] -= 1
        return "".join(res)


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        str1 = input()
        solObj = Solution()
        str2 = solObj.rearrangeString(str1)
        if str2 == "-1":
            print(0)
        elif sorted(str1) != sorted(str2):
            print(0)
        else:
            for i in range(len(str2) - 1):
                if str2[i] == str2[i + 1]:
                    print(0)
                    break
            else:
                print(1)

# } Driver Code Ends
