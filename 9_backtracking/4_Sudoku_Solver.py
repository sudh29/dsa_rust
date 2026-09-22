def find_next_empty(puzzle):
    for i in range(9):
        for j in range(9):
            if puzzle[i][j] == 0:
                return i, j
    return None, None


def is_valid(grid, guess, row, col):
    row_vals = grid[row]
    if guess in row_vals:
        return False
    col_vals = [grid[i][col] for i in range(9)]
    if guess in col_vals:
        return False

    row_st = (row // 3) * 3
    col_st = (col // 3) * 3
    for r in range(row_st, row_st + 3):
        for c in range(col_st, col_st + 3):
            if grid[r][c] == guess:
                return False
    return True


class Solution:
    def SolveSudoku(self, grid):
        row, col = find_next_empty(grid)
        if row is None:
            return True
        if grid[row][col] == 0:
            for guess in range(1, 10):
                if is_valid(grid, guess, row, col):
                    grid[row][col] = guess
                    if self.SolveSudoku(grid):
                        return True
                    grid[row][col] = 0
            return False
        return True

    def printGrid(self, arr):
        for row in range(9):
            for col in range(9):
                print(arr[row][col], end=" ")


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())
    while t > 0:
        grid = [[0 for i in range(9)] for j in range(9)]
        row = [int(x) for x in input().strip().split()]
        k = 0
        for i in range(9):
            for j in range(9):
                grid[i][j] = row[k]
                k += 1

        ob = Solution()

        if ob.SolveSudoku(grid) == True:
            ob.printGrid(grid)
            print()
        else:
            print("No solution exists")
        t = t - 1
# } Driver Code Ends
