# shortest path to all nodes

INF = float("inf")


# floyd warshall
def printmat(m):
    r, c = len(m), len(m[0])
    for i in range(r):
        for j in range(c):
            print(m[i][j], end=" ")
        print()


v, e = 4, 4
m = [[None] * v for i in range(v)]
for i in range(v):
    for j in range(v):
        if i == j:
            m[i][j] = 0
        else:
            m[i][j] = INF
m[0][3] = 10
m[0][1] = 5
m[1][2] = 3
m[2][3] = 1

print()
printmat(m)
print()
for k in range(v):
    for i in range(v):
        for j in range(v):
            if m[i][k] + m[k][j] < m[i][j]:
                m[i][j] = m[i][k] + m[k][j]

printmat(m)
