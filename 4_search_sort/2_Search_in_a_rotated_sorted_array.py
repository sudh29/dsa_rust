class Solution:
    def search(self, nums: list[int], target: int) -> int:
        start, end = 0, len(nums) - 1

        while start <= end:
            mid = (start + end) // 2

            if nums[mid] == target:
                return mid

            if nums[start] <= nums[mid]:  # Left sorted
                if target >= nums[start] and target < nums[mid]:
                    end = mid - 1
                else:
                    start = mid + 1
            else:  # Right sorted
                if target > nums[mid] and target <= nums[end]:
                    start = mid + 1
                else:
                    end = mid - 1

        return -1


# Example usage:
# sol = Solution()
# print(sol.search([4,5,6,7,0,1,2], 0))  # Output: 4
# print(sol.search([4,5,6,7,0,1,2], 3))  # Output: -1
