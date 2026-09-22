# {
# Driver Code Starts
# Initial Template for Python 3

# } Driver Code Ends


def solve(nums, index, res):
    if index == len(nums) - 1:
        res.append("".join(nums))
        return
    seen = set()
    for i in range(index, len(nums)):
        if nums[i] not in seen:
            seen.add(nums[i])
            nums[index], nums[i] = nums[i], nums[index]
            solve(nums, index + 1, res)
            nums[index], nums[i] = nums[i], nums[index]


def factorial(n):
    if n == 0 or n == 1:
        return 1
    return n * factorial(n - 1)


class Solution:
    def kthPermutation(self, n: int, k: int) -> str:
        # numbers = [str(i) for i in range(1,n+1)]
        # res = []
        # solve(numbers,0,res)
        # totalPermutations = factorial(n)
        # res = sorted(res)
        # if k > totalPermutations:
        #     k = k % totalPermutations
        # return res[k-1]

        totalPermutations = factorial(n)
        if k > totalPermutations:
            k = k % totalPermutations

        nums = [str(i) for i in range(1, n + 1)]
        result = []
        k -= 1
        while n > 0:
            index = k // factorial(n - 1)
            result.append(nums.pop(index))
            k %= factorial(n - 1)
            n -= 1
        return "".join(result)


# {
# Driver Code Starts.
if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        N, K = map(int, input().split())

        obj = Solution()
        res = obj.kthPermutation(N, K)

        print(res)


# } Driver Code Ends
