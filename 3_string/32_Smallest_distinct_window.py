class Solution:
    def findSubString(self, str):
        # if len(str)==1:
        #     return 1
        # char_set = set()
        # for i in str:
        #     char_set.add(i)
        # window_size = len(char_set)
        # while window_size<=len(str):
        #     for i in range(len(str)-window_size+1):
        #         temp = set(str[i:i+window_size])
        #         if (temp)==(char_set):
        #             return window_size
        #     window_size+=1
        # return 0

        n = len(str)
        if n == 0:
            return 0
        unique_chars = set(str)
        required_char_count = len(unique_chars)

        char_count = {}
        min_length = float("inf")
        left = 0
        unique_chars_in_window = 0
        for right in range(n):
            char = str[right]
            char_count[char] = char_count.get(char, 0) + 1
            if char_count[char] == 1:
                unique_chars_in_window += 1
            while unique_chars_in_window == required_char_count:
                current_window_length = right - left + 1
                if current_window_length < min_length:
                    min_length = current_window_length
                char_count[str[left]] -= 1
                if char_count[str[left]] == 0:
                    unique_chars_in_window -= 1
                left += 1
        return min_length if min_length != float("inf") else 0


# {
# Driver Code Starts
# Initial Template for Python 3


def main():
    T = int(input())

    while T > 0:
        str = input()
        ob = Solution()
        print(ob.findSubString(str))

        T -= 1


if __name__ == "__main__":
    main()

# } Driver Code Ends
