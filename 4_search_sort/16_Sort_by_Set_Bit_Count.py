class Solution:
    # Function to count set bits (1s in binary) in a number
    @staticmethod
    def count_set_bits(x):
        return bin(x).count("1")

    # Custom comparison function for sorting
    @staticmethod
    def comp(a, b):
        return Solution.count_set_bits(a) > Solution.count_set_bits(b)

    # Function to sort the array by set bit count
    def sortBySetBitCount(self, arr, n):
        # Sorting based on set bit count while maintaining relative order (stable sort)
        arr.sort(key=lambda x: (-Solution.count_set_bits(x)))
