class Solution:
    # arr: input list of integers
    # total_k: the quadruple sum required
    def fourSum(self, arr, total_k):
        arr.sort()  # Sort the array
        n = len(arr)
        res = []
        st1 = set()  # Using a set to avoid duplicate quadruples

        for i in range(n - 3):
            for j in range(i + 1, n - 2):
                k = j + 1
                l = n - 1
                while l > k:
                    current_sum = arr[i] + arr[j] + arr[k] + arr[l]
                    if current_sum == total_k:
                        temp = [arr[i], arr[j], arr[k], arr[l]]
                        st1.add(tuple(temp))  # Use a tuple to store in a set
                        k += 1
                        l -= 1
                    elif current_sum < total_k:
                        k += 1
                    else:
                        l -= 1

        # Converting the set back to a list of lists
        res = [list(item) for item in st1]
        return res
