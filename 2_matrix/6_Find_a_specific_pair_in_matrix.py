def find_max_value(mat):
    n = len(mat)

    # Create the max_matrix and initialize it with 0s
    max_matrix = [[0 for _ in range(n)] for _ in range(n)]

    # Last element of max_matrix is the same as the last element of mat
    max_matrix[n - 1][n - 1] = mat[n - 1][n - 1]

    # Fill the last row and last column of max_matrix
    max_value = mat[n - 1][n - 1]
    for i in range(n - 2, -1, -1):
        if mat[n - 1][i] > max_value:
            max_value = mat[n - 1][i]
        max_matrix[n - 1][i] = max_value

    max_value = mat[n - 1][n - 1]
    for i in range(n - 2, -1, -1):
        if mat[i][n - 1] > max_value:
            max_value = mat[i][n - 1]
        max_matrix[i][n - 1] = max_value

    # Fill the rest of max_matrix
    for i in range(n - 2, -1, -1):
        for j in range(n - 2, -1, -1):
            max_matrix[i][j] = max(
                mat[i][j],
                max_matrix[i + 1][j],
                max_matrix[i][j + 1],
                max_matrix[i + 1][j + 1],
            )

    # Now find the maximum value of mat[c][d] - mat[a][b]
    max_diff = float("-inf")
    for i in range(n - 1):
        for j in range(n - 1):
            if max_matrix[i + 1][j + 1] - mat[i][j] > max_diff:
                max_diff = max_matrix[i + 1][j + 1] - mat[i][j]

    return max_diff


# Example usage
mat = [
    [1, 2, -1, -4, -20],
    [-8, -3, 4, 2, 1],
    [3, 8, 6, 1, 3],
    [-4, -1, 1, 7, -6],
    [0, -4, 10, -5, 1],
]

print(find_max_value(mat))  # Output should be 18
