class Solution:
    # Function to find the next greater element for each element of the array.
    def nextLargerElement(self, arr, n):
        res = [-1] * n  # Initialize result array with -1
        s = []  # Stack to hold indices
        for i in range(n):
            while (
                s and arr[i] > arr[s[-1]]
            ):  # While stack is not empty and current element is greater
                idx = s.pop()  # Get the index from stack
                res[idx] = arr[i]  # Set the next greater element
            s.append(i)  # Push current index onto the stack
        return res
