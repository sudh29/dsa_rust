class Solution:
    def countSquares(self, N: int) -> int:
        if N <= 1:
            return 0
        if N < 4:
            return 1

        count = 0
        for i in range(1, N):
            if i * i < N:
                count += 1
            else:
                break

        return count


# Example usage:
# sol = Solution()
# print(sol.countSquares(10))  # Output: 3 (1, 4, 9)
