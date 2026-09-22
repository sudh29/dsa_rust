class Solution:
    def findTrailingZeros(self, n: int) -> int:
        count = 0
        i = 5
        while i <= n:
            count += n // i
            i *= 5
        return count

    def findNum(self, n: int) -> int:
        low = 5
        high = 1_000_000_000
        while low <= high:
            mid = low + (high - low) // 2
            ans = self.findTrailingZeros(mid)
            if ans == n:
                return (mid // 5) * 5
            elif ans > n:
                high = mid - 1
            else:
                low = mid + 1


# Example usage:
# sol = Solution()
# print(sol.findNum(5))  # Output: 25 (5! has 6 trailing zeros)
# print(sol.findNum(1))  # Output: 5 (1! has 0 trailing zeros)
