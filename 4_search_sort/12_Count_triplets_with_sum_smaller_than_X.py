class Solution:
    # Function to count triplets with sum smaller than the given value
    def countTriplets(self, arr, n, target_sum):
        arr.sort()  # Sorting the array
        count = 0

        for i in range(n - 2):
            temp_sum = target_sum - arr[i]
            j = i + 1
            k = n - 1

            while k > j:
                if arr[j] + arr[k] < temp_sum:
                    count += k - j
                    j += 1
                else:
                    k -= 1

        return count
