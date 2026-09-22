# User function Template for python3


def solve(arr, target, current, idx, result):
    if target == 0:
        # if current[:] not in result:
        result.append(current[:])
        return
    for i in range(idx, len(arr)):
        if arr[i] > target:
            break
        if i > idx and arr[i] == arr[i - 1]:
            continue
        current.append(arr[i])
        solve(arr, target - arr[i], current, i, result)
        current.pop()


class Solution:
    # Function to return a list of indexes denoting the required
    # combinations whose sum is equal to given number.
    def combinationalSum(self, A, B):
        A.sort()
        result = []
        solve(A, B, [], 0, result)
        return result


# {
# Driver Code Starts.


if __name__ == "__main__":
    test_cases = int(input())
    for cases in range(test_cases):
        n = int(input())
        a = list(map(int, input().strip().split()))
        s = int(input())
        ob = Solution()
        result = ob.combinationalSum(a, s)
        if not len(result):
            print("Empty")
            continue
        for i in range(len(result)):
            print("(", end="")
            size = len(result[i])
            for j in range(size - 1):
                print(result[i][j], end=" ")
            if size:
                print(result[i][size - 1], end=")")
            else:
                print(")", end="")
        print()

# } Driver Code Ends
