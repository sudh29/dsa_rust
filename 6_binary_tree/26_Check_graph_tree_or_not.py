class Solution:
    def isTree(self, n, adj):
        visited = [False] * n
        if self.isCyclicUtil(0, visited, -1, adj):
            return 0
        # for i in range(n):
        #     if visited[i] == False:
        #         return 0
        # return 1
        return 1 if all(visited) else 0

    def isCyclicUtil(self, curr, visited, parent, adj):
        visited[curr] = True
        for i in adj[curr]:
            if visited[i] == False:
                if self.isCyclicUtil(i, visited, curr, adj):
                    return True
            elif i != parent:
                return True
        return False
