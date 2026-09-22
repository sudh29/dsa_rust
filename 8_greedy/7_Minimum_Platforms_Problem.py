# User function Template for python3


class Solution:
    # Function to find the minimum number of platforms required at the
    # railway station such that no train waits.
    def minimumPlatform(self, n, arr, dep):
        plat_needed = 1
        result = 1
        # for i in range(n):
        #     plat_needed = 1
        #     for j in range(n):
        #         if i != j:
        #             if (arr[i] >= arr[j] and dep[j] >= arr[i]):
        #                 plat_needed += 1
        #     result = max(result, plat_needed)
        # return result

        arr.sort()
        dep.sort()
        i = 1
        j = 0
        # two pointer approach
        while i < n and j < n:
            # print(arr[i] , dep[j])
            if arr[i] > dep[j]:
                j += 1
                plat_needed -= 1
            else:
                plat_needed += 1
                i += 1
            result = max(result, plat_needed)
        return result


# {
# Driver Code Starts
# Initial Template for Python 3

# Contributed by : Nagendra Jha


if __name__ == "__main__":
    test_cases = int(input())
    for cases in range(test_cases):
        n = int(input())
        arrival = list(map(int, input().strip().split()))
        departure = list(map(int, input().strip().split()))
        ob = Solution()
        print(ob.minimumPlatform(n, arrival, departure))
# } Driver Code Ends
