class Solution:
    def findMaxProduct(self, a, n):
        if n == 1:
            return a[0]
        mod = 10**9 + 7
        zero_count = 0
        negative_count = 0
        prod = 1
        max_negative = float("-inf")
        for i in a:
            if i == 0:
                zero_count += 1
                continue
            if i < 0:
                negative_count += 1
                max_negative = max(max_negative, i)
            prod = (prod * i) % mod

        if zero_count == n or (
            negative_count == 1 and negative_count + zero_count == n
        ):
            return 0
        if negative_count % 2 != 0:
            prod = (prod * pow(max_negative, mod - 2, mod)) % mod
        return prod


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    for _ in range(int(input())):
        n = int(input())
        a = list(map(int, input().split()))
        obj = Solution()
        ans = obj.findMaxProduct(a, n)
        print(ans)
# } Driver Code Ends
