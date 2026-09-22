"""
class Node:
    def __init__(self, val):
        self.right = None
        self.data = val
        self.left = None
"""


class Solution:
    def sumK(self, root, k):
        def solve(root, path, k, count, curr_sum):
            if root is None:
                return
            curr_sum += root.data
            if curr_sum - k in path:
                count[0] += path[curr_sum - k]
            path[curr_sum] = path.get(curr_sum, 0) + 1

            solve(root.left, path, k, count, curr_sum)
            solve(root.right, path, k, count, curr_sum)
            path[curr_sum] -= 1

        path = {0: 1}
        count = [0]
        solve(root, path, k, count, 0)
        return count[0]

        # def solve(root,path,k,count):
        #     if root is None:
        #         return
        #     path.append(root.data)
        #     solve(root.left,path,k,count)
        #     solve(root.right,path,k,count)
        #     sum_val = 0
        #     for j in range(len(path) - 1, -1, -1):
        #         sum_val += path[j]
        #         if (sum_val == k):
        #             count[0]+=1
        #     path.pop(-1)

        # path=[]
        # count = [0]
        # solve(root,path,k,count)
        # return count[0]
