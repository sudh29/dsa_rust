# find path using backtracking


def pathFinder(mat, pos, n):
    if pos == (n - 1, n - 1):
        return (n - 1, n - 1)
    x, y = pos
    if x + 1 < n and mat[x + 1][y] == 1:
        a = pathFinder(mat, (x + 1, y), n)
        if a != None:
            return (x, y) + a
    if y + 1 < n and mat[x][y + 1] == 1:
        b = pathFinder(mat, (x, y + 1), n)
        if b != None:
            return (x, y) + b


matrix = [
    [1, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 0, 0],
    [1, 1, 1, 1, 1],
]
print(pathFinder(matrix, (0, 0), 5))
