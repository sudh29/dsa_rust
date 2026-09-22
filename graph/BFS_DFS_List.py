# Adjacency List Graph Class
class ListGraph:
    # Constructor
    def __init__(self, nodes):
        self.data = {}
        for node in nodes:
            self.data[node] = []

    # Add the vertex in the Graph
    def add_vertex(self, u, v):
        self.data[u].append(v)
        self.data[v].append(u)

    # Print graph
    def print_graph(self):
        for i in self.data:
            print(i, self.data[i])
        # print(self.data)

    # DFS
    def dfs(self, source, nodes):
        path = []
        visited = {}
        s = [source]

        for node in nodes:
            visited[node] = 0

        visited[source] = 1

        while len(s) > 0:
            v = s.pop()
            path.append(v)
            for i in self.data[v]:
                if visited[i] == 0:
                    visited[i] = 1
                    s.append(i)
        return path

    # BFS
    def bfs(self, source, nodes):
        path = []
        visited = {}
        q = [source]

        for node in nodes:
            visited[node] = 0

        visited[source] = 1

        path.append(source)

        while len(q) > 0:
            v = q.pop(0)
            for i in range(len(self.data[v])):
                if visited[self.data[v][i]] == 0:
                    q.append(self.data[v][i])
                    visited[self.data[v][i]] = 1
                    path.append(self.data[v][i])
        return path


AdjacencyListGraph = ListGraph(["A", "B", "C", "D", "E", "F"])
AdjacencyListGraph.print_graph()
print()
AdjacencyListGraph.add_vertex("A", "B")
AdjacencyListGraph.add_vertex("A", "C")
AdjacencyListGraph.add_vertex("B", "D")
AdjacencyListGraph.add_vertex("C", "E")
AdjacencyListGraph.add_vertex("E", "B")
AdjacencyListGraph.add_vertex("D", "E")
AdjacencyListGraph.add_vertex("D", "F")
AdjacencyListGraph.add_vertex("E", "F")
AdjacencyListGraph.print_graph()
print()
print("DFS")
print(AdjacencyListGraph.dfs("A", ["A", "B", "C", "D", "E", "F"]))

print()
print("BFS")
print(AdjacencyListGraph.bfs("A", ["A", "B", "C", "D", "E", "F"]))
