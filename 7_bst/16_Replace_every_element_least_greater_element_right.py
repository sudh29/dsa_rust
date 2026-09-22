from typing import List


class Node:
    def __init__(self, val):
        self.right = None
        self.data = val
        self.left = None


class Solution:
    def findLeastGreater(self, n: int, arr: List[int]) -> List[int]:
        def solve(node, data, ans):
            if node is None:
                node = Node(data)
                return node
            if data < node.data:
                ans.append(node.data)
                node.left = solve(node.left, data, ans)
            elif data >= node.data:
                node.right = solve(node.right, data, ans)
            return node

        n = len(arr)
        # for i in range(n):
        #     min_val = float("inf")
        #     for j in range(i+1,n):
        #         if arr[j]>arr[i]:
        #             min_val = min(min_val,arr[j])
        #     if min_val==float("inf"):
        #         arr[i] = -1
        #     else:
        #         arr[i] = min_val
        # return arr
        root = None
        for i in range(n - 1, -1, -1):
            ans = []
            root = solve(root, arr[i], ans)
            if ans:
                arr[i] = ans.pop()
            else:
                arr[i] = -1
        return arr
