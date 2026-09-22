class Solution:
    def inSequence(self, A: int, B: int, C: int) -> int:
        if C == 0:
            return int(A == B)  # Return 1 if A equals B, else return 0
        d = (B - A) // C
        r = (B - A) % C
        return int(
            d >= 0 and r == 0
        )  # Return 1 if both conditions are satisfied, else return 0


# Example usage:
# sol = Solution()
# print(sol.inSequence(1, 5, 2))  # Output: 0 (5 is not in the sequence)
# print(sol.inSequence(1, 1, 0))  # Output: 1 (A equals B)
# print(sol.inSequence(1, 3, 1))  # Output: 1 (3 is in the sequence)
