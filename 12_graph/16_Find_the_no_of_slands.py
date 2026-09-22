import sys

sys.setrecursionlimit(10**8)


def dfs(row, col, grid):
    if (
        row < 0
        or col < 0
        or row >= len(grid)
        or col >= len(grid[0])
        or grid[row][col] != 1
    ):
        return
    grid[row][col] = -1

    dfs(row + 1, col, grid)
    dfs(row - 1, col, grid)
    dfs(row, col + 1, grid)
    dfs(row, col - 1, grid)
    dfs(row + 1, col + 1, grid)
    dfs(row + 1, col - 1, grid)
    dfs(row - 1, col + 1, grid)
    dfs(row - 1, col - 1, grid)


class Solution:
    def numIslands(self, grid):
        n = len(grid)
        m = len(grid[0])

        num_islands = 0
        for i in range(n):
            for j in range(m):
                if grid[i][j] == 1:
                    dfs(i, j, grid)
                    num_islands += 1
        return num_islands


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    for _ in range(int(input())):
        n, m = map(int, input().strip().split())
        grid = []
        for i in range(n):
            grid.append([int(i) for i in input().strip().split()])
        obj = Solution()
        print(obj.numIslands(grid))
# } Driver Code Ends
