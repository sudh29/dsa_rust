from typing import List


class Solution:
    def minSwaps(self, n: int, A: List[int]) -> int:
        def cal_min_swaps(arr, temp, n):
            swaps = 0
            visited = [False] * n
            for i in range(n):
                if visited[i] or arr[i] == temp[i]:
                    continue
                cycle_size = 0
                j = i
                while not visited[j]:
                    visited[j] = True
                    j = arr.index(temp[j])
                    cycle_size += 1
                if cycle_size > 0:
                    swaps += cycle_size - 1
            return swaps

        def inorder(a, n, index, ans):
            if index >= n:
                return
            inorder(a, n, 2 * index + 1, ans)
            ans.append(a[index])
            inorder(a, n, 2 * index + 2, ans)

        tree = []
        inorder(A, n, 0, tree)
        # print(tree) binary tree inorder
        # print(A)    BST tree inorder
        res = cal_min_swaps(tree, A, n)
        return res
