# Adjacency Matrix Graph Class
class Graph:
    # Constructor
    def __init__(self, size):
        self.data = []
        for _ in range(size):
            temp = []
            for j in range(size):
                temp.append(0)
            self.data.append(temp)

    # Add the vertex in the graph
    def add_vertex(self, u, v):
        self.data[u][v] = 1

    # Print graph
    def print_graph(self):
        for i in self.data:
            print(i)

    # BFS
    def bfs(self, source):
        path = []
        visited = []
        q = [source]

        for i in range(len(self.data)):
            visited.append(0)

        visited[source] = 1
        path.append(source)

        while len(q) > 0:
            v = q.pop(0)
            for i in range(len(self.data[v])):
                if visited[i] == 0 and self.data[v][i] == 1:
                    q.append(i)
                    visited[i] = 1
                    path.append(i)
        return path


AdjacencyMatrixGraph = Graph(8)
AdjacencyMatrixGraph.print_graph()
print()
AdjacencyMatrixGraph.add_vertex(0, 1)
AdjacencyMatrixGraph.add_vertex(0, 2)
AdjacencyMatrixGraph.add_vertex(0, 3)
AdjacencyMatrixGraph.add_vertex(1, 4)
AdjacencyMatrixGraph.add_vertex(1, 5)
AdjacencyMatrixGraph.add_vertex(2, 6)
AdjacencyMatrixGraph.add_vertex(2, 7)
AdjacencyMatrixGraph.add_vertex(3, 7)

listA = [0, 1, 2, 3, 4, 5, 6, 7]
print(listA)
AdjacencyMatrixGraph.print_graph()
print()
print(AdjacencyMatrixGraph.bfs(0))
