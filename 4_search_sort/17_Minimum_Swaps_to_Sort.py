class Solution:
    # Function to find the minimum number of swaps required to sort the array
    def minSwaps(self, nums):
        n = len(nums)

        # Create a list of pairs (value, original index)
        temp = [(nums[i], i) for i in range(n)]

        # Sort the list based on values
        temp.sort()

        i = 0
        c = 0

        while i < n:
            idx = temp[i][1]

            if idx == i:  # Already in the right position
                i += 1
            else:
                # Swap the current element with the element at its correct position
                temp[i], temp[idx] = temp[idx], temp[i]

                # If swapped to the start, reset i to 0, else move i back
                i = max(0, i - 1)
                c += 1

        return c
