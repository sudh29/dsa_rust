# User function Template for python3


class Solution:
    def maxSubArraySum(self, arr):
        max_so_far = arr[0]
        current_max = arr[0]
        for i in range(1, len(arr)):
            current_max = max(arr[i], current_max + arr[i])
            max_so_far = max(max_so_far, current_max)
        return max_so_far


# {
# Driver Code Starts
# Initial Template for Python 3


def main():
    T = int(input())
    while T > 0:
        arr = [int(x) for x in input().strip().split()]

        ob = Solution()

        print(ob.maxSubArraySum(arr))

        T -= 1


if __name__ == "__main__":
    main()

# } Driver Code End
