def valid(node, graph, color, c, graph_len):
    for i in range(graph_len):
        if graph[node][i] and color[i] == c:
            return False
    return True


def solve(node, m, color, graph, graph_len):
    if node == graph_len:
        return True
    for c in range(1, m + 1):
        if valid(node, graph, color, c, graph_len):
            color[node] = c
            if solve(node + 1, m, color, graph, graph_len):
                return True
            color[node] = 0
    return False


# Function to determine if graph can be coloured with at most M colours such
# that no two adjacent vertices of graph are coloured with same colour.
def graphColoring(graph, k, V):
    # DFS + Backtracking
    color = [0] * V
    if solve(0, k, color, graph, V):
        return 1
    return 0


# {
# Driver Code Starts
# Initial Template for Python 3

if __name__ == "__main__":
    t = int(input())
    while t > 0:
        V = int(input())
        k = int(input())
        m = int(input())
        l = [int(x) for x in input().strip().split()]
        graph = [[0 for i in range(V)] for j in range(V)]
        cnt = 0
        for i in range(m):
            graph[l[cnt] - 1][l[cnt + 1] - 1] = 1
            graph[l[cnt + 1] - 1][l[cnt] - 1] = 1
            cnt += 2
        if graphColoring(graph, k, V) == True:
            print(1)
        else:
            print(0)

        t = t - 1
