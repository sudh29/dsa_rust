# User function Template for python3


def is_valid(x, y, n, m):
    return 0 <= x < n and 0 <= y < m


def search_from(x, y, n, m, word_len):
    directions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)]
    for dx, dy in directions:
        nx, ny = x, y
        match = True
        for k in range(word_len):
            if not is_valid(nx, ny, n, m) or grid[nx][ny] != word[k]:
                match = False
                break
            nx += dx
            ny += dy
        if match:
            return True
    return False


class Solution:
    def searchWord(self, grid, word):
        n = len(grid)
        m = len(grid[0])
        word_len = len(word)
        result = []
        for i in range(n):
            for j in range(m):
                if grid[i][j] == word[0] and search_from(i, j, n, m, word_len):
                    result.append((i, j))
        result.sort()
        return result


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    T = int(input())
    for i in range(T):
        n, m = input().split()
        n = int(n)
        m = int(m)
        grid = []
        for _ in range(n):
            cur = input()
            temp = []
            for __ in cur:
                temp.append(__)
            grid.append(temp)
        word = input()
        obj = Solution()
        ans = obj.searchWord(grid, word)
        for _ in ans:
            for __ in _:
                print(__, end=" ")
            print()
        if len(ans) == 0:
            print(-1)

# } Driver Code Ends
