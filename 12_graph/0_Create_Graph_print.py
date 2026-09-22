class Graph:
    def __init__(self):
        self.adjlist = {}

    def add_edge(self, u, v, dist, bidirec=True):
        if u not in self.adjlist:
            self.adjlist[u] = []
        self.adjlist[u].append((v, dist))

        if bidirec:
            if v not in self.adjlist:
                self.adjlist[v] = []
            self.adjlist[v].append((u, dist))

    def print_adj(self):
        for u, neighbors in self.adjlist.items():
            print(u, ":", end=" ")
            for v, dist in neighbors:
                print("(", v, ",", dist, ")", end=" ")
            print()


if __name__ == "__main__":
    g = Graph()
    g.add_edge("0", "1", 4, False)
    g.add_edge("0", "7", 8, False)
    g.add_edge("1", "7", 11, False)
    g.add_edge("1", "2", 8, False)
    g.add_edge("7", "8", 7, False)
    g.add_edge("2", "8", 2, False)
    g.add_edge("8", "6", 6, False)
    g.add_edge("2", "5", 4, False)
    g.add_edge("6", "5", 2, False)
    g.add_edge("2", "3", 7, False)
    g.add_edge("3", "3", 14, False)
    g.add_edge("3", "4", 9, False)
    g.add_edge("5", "4", 10, False)
    g.add_edge("7", "6", 1, False)
    g.print_adj()
