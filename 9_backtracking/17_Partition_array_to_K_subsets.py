# User function Template for python3
def solve(a, n, k, curr_sum, count, visited, sub_set_sum, idx):
    if sub_set_sum == curr_sum:
        if count == k - 2:
            return True
        return solve(a, n, k, 0, count + 1, visited, sub_set_sum, n - 1)

    for i in range(idx, -1, -1):
        if visited[i] or curr_sum + a[i] > sub_set_sum:
            continue
        visited[i] = True
        if solve(a, n, k, curr_sum + a[i], count, visited, sub_set_sum, i - 1):
            return True
        visited[i] = False
    return False


class Solution:
    def isKPartitionPossible(self, a, k):
        n = len(a)
        if k == 1:
            return True
        if n < k:
            return False
        total_sum = sum(a)
        if total_sum % k != 0:
            return False

        target_sum = total_sum // k
        visited = [False] * n
        curr_sum = 0
        count = 0
        return solve(a, n, k, curr_sum, count, visited, target_sum, n - 1)


# {
# Driver Code Starts


if __name__ == "__main__":
    tcs = int(input())
    for _ in range(tcs):
        N = int(input())
        arr = [int(x) for x in input().split()]
        k = int(input())
        if Solution().isKPartitionPossible(arr, k):
            print(1)
        else:
            print(0)
# } Driver Code Ends
