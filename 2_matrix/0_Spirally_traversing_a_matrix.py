class Solution:
    # Function to return a list of integers denoting spiral traversal of matrix.
    def spirallyTraverse(self, matrix, r, c):
        # code here
        temp = []
        left = 0
        right = c - 1
        top = 0
        bottom = r - 1
        while left <= right and top <= bottom:
            for j in range(left, right + 1):
                temp.append(matrix[top][j])
            top += 1
            for j in range(top, bottom + 1):
                temp.append(matrix[j][right])
            right -= 1
            if top <= bottom:
                for j in range(right, left - 1, -1):
                    temp.append(matrix[bottom][j])
                bottom -= 1
            if left <= right:
                for j in range(bottom, top - 1, -1):
                    temp.append(matrix[j][left])
                left += 1
        return temp
