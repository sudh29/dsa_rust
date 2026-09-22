class Solution:
    def merge(self, arr: list[int], l: int, m: int, r: int) -> int:
        temp = []
        i, j, ci = l, m + 1, 0

        while i <= m and j <= r:
            if arr[i] <= arr[j]:
                temp.append(arr[i])
                i += 1
            else:
                temp.append(arr[j])
                ci += m - i + 1
                j += 1

        while i <= m:
            temp.append(arr[i])
            i += 1

        while j <= r:
            temp.append(arr[j])
            j += 1

        for i in range(len(temp)):
            arr[l + i] = temp[i]

        return ci

    def mergesort(self, arr: list[int], low: int, high: int) -> int:
        ci = 0
        if low < high:
            mid = (low + high) // 2
            ci += self.mergesort(arr, low, mid)
            ci += self.mergesort(arr, mid + 1, high)
            ci += self.merge(arr, low, mid, high)
        return ci

    def inversionCount(self, arr: list[int], N: int) -> int:
        return self.mergesort(arr, 0, N - 1)


# Example usage:
# sol = Solution()
# print(sol.inversionCount([1, 3, 2, 3, 1], 5))  # Output: 4
