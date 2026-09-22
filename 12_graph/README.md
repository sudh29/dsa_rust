# Graph Problems with Patterns

| File | Pattern(s) |
|------|------------|
| [0_Create_Graph_print.py](0_Create_Graph_print.py) | Graph Construction / Adjacency List |
| [1_Create_Graph.py](1_Create_Graph.py) | Graph Construction / Edge List |
| [2_Implement_BFS_algorithm.py](2_Implement_BFS_algorithm.py) | BFS / Queue / Graph Traversal |
| [3_Implement_DFS_Algo.py](3_Implement_DFS_Algo.py) | DFS / Recursion / Stack |
| [4_Detect_Cycle_Directed_Graph.py](4_Detect_Cycle_Directed_Graph.py) | DFS / Cycle Detection / Directed Graph |
| [5_Detect_Cycle_UnDirected_Graph.py](5_Detect_Cycle_UnDirected_Graph.py) | DFS / Cycle Detection / Union-Find |
| [6_Search_in_Maze.py](6_Search_in_Maze.py) | BFS / DFS / Matrix Graph |
| [7_Minimum_Step_by_Knight.py](7_Minimum_Step_by_Knight.py) | BFS / Shortest Path |
| [8_Flood_fill_algo.py](8_Flood_fill_algo.py) | DFS / BFS / Matrix Traversal |
| [9_Clone_a_graph.py](9_Clone_a_graph.py) | DFS / BFS / Graph Cloning / HashMap |
| [10_Making_wired_Connections.py](10_Making_wired_Connections.py) | Union-Find / Disjoint Set / Connectivity |
| [12_Dijkstra_algo.py](12_Dijkstra_algo.py) | Dijkstra's Algorithm / Priority Queue / Shortest Path |
| [13_Implement_Topological_Sort.py](13_Implement_Topological_Sort.py) | Topological Sort / Kahn's Algo / DFS |
| [14_Minimum_time_taken_job_completed_Directed_Acyclic_Graph.py](14_Minimum_time_taken_job_completed_Directed_Acyclic_Graph.py) | Topological Sort / DAG Scheduling |
| [16_Find_the_no_of_slands.py](16_Find_the_no_of_slands.py) | DFS / BFS / Connected Components |
| [18_Implement_Kruskals_Algorithm.py](18_Implement_Kruskals_Algorithm.py) | Kruskal's Algorithm / Union-Find / MST |
| [36_M-Colouring_Problem.py](36_M-Colouring_Problem.py) | Backtracking / Graph Coloring |

---

## Graph Traversal: DFS and BFS

Graph traversal techniques help explore all nodes of a graph:

### Depth First Search (DFS)
- Explores as far along each branch before backtracking.
- Can be implemented with recursion or a stack.
- Useful for cycle detection, component detection.

### Breadth First Search (BFS)
- Explores all neighbors first before moving to next level.
- Uses a queue.
- Helpful in shortest path problems in unweighted graphs.

---

## DFS and BFS in Python (Adjacency List)

```python
from collections import defaultdict

class Graph():
    def __init__(self):
        self.graph = defaultdict(list)

    def add_edge(self, u, v):
        self.graph[u].append(v)

    def print_graph(self):
        print(self.graph)

def solve_dfs(val, visited, graph):
    visited[val] = True
    print(val, end=' ')
    for i in graph[val]:
        if not visited[i]:
            solve_dfs(i, visited, graph)

def dfs(val, graph):
    visited = [False] * len(graph)
    solve_dfs(val, visited, graph)

def bfs(val, graph):
    visited = [False] * len(graph)
    q = [val]
    visited[val] = True
    while q:
        node = q.pop(0)
        print(node, end=' ')
        for i in graph[node]:
            if not visited[i]:
                q.append(i)
                visited[i] = True

# Example usage:
g = Graph()
g.add_edge(0, 1)
g.add_edge(0, 2)
g.add_edge(1, 2)
g.add_edge(2, 0)
g.add_edge(2, 3)
g.add_edge(3, 3)

g.print_graph()          # Output: defaultdict(<class 'list'>, {0: [1, 2], 1: [2], 2: [0, 3], 3: [3]})
dfs(2, g.graph)          # Output: 2 0 1 3
print()
bfs(2, g.graph)          # Output: 2 0 3 1
```


In a graph, Depth First Search (DFS) and Breadth First Search (BFS) algorithms are used to traverse the nodes. Here's how they work:

# Depth First Search (DFS):

DFS explores as far as possible along each branch before backtracking. It starts at a source node and explores as far as possible along each branch before backtracking.
DFS can be implemented using recursion or a stack data structure.
It visits all the vertices of a graph and for each vertex, it explores all the edges adjacent to that vertex.
DFS is often used to detect cycles in a graph.

# Breadth First Search (BFS):

BFS explores neighbors of the current vertex before moving to the next level of vertices. It starts at a source node and explores all the neighbor nodes at the present depth before moving on to the nodes at the next depth level.
BFS uses a queue data structure to keep track of the nodes to be visited next.
BFS is often used to find the shortest path between two nodes in an unweighted graph.

## Below are the Python implementations of DFS and BFS for a graph represented using an adjacency list:

# Code

```python
from collections import defaultdict

class Graph():
    def __init__(self):
        self.graph = defaultdict(list)

    def add_edge(self,u,v):
        self.graph[u].append(v)

    def print_graph(self):
        print(self.graph)

def solve_dfs(val,visited,graph):
    visited[val] = True
    print(val,end=' ')
    for i in graph[val]:
        if not visited[i]:
            solve_dfs(i,visited,graph)

def dfs(val,graph):
    visited = [False]*len(graph)
    solve_dfs(val,visited,graph)

def dfs(start_node, graph):
    stack = [start_node]
    visited = [False] * len(graph)
    visited[start_node] = True

    while stack:
        current_node = stack.pop()
        print(current_node, end=' ')

        for neighbor in graph[current_node]:
            if not visited[neighbor]:
                stack.append(neighbor)
                visited[neighbor] = True

def bfs(val,graph):
    visited = [False]*len(graph)
    q = []
    q.append(val)
    visited[val]=True
    while q:
        val = q.pop(0)
        print(val,end=' ')
        for i in graph[val]:
            if not visited[i]:
                q.append(i)
                visited[i]=True

# Example usage:
g = Graph()
g.add_edge(0, 1)
g.add_edge(0, 2)
g.add_edge(1, 2)
g.add_edge(2, 0)
g.add_edge(2, 3)
g.add_edge(3, 3)

g.print_graph() # defaultdict(<class 'list'>, {0: [1, 2], 1: [2], 2: [0, 3], 3: [3]})
dfs(2,g.graph) # 2 0 1 3
print()
bfs(2,g.graph) # 2 0 3 1

```
