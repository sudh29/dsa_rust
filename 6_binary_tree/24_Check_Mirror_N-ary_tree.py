class Solution:
    def checkMirrorTree(self, n, e, A, B):
        dict_map = {}
        for i in range(0, 2 * e, 2):
            if A[i] not in dict_map.keys():
                dict_map[A[i]] = [A[i + 1]]
            else:
                dict_map[A[i]].append(A[i + 1])
        # print(dict_map)
        for i in range(0, 2 * e, 2):
            if dict_map[B[i]][-1] != B[i + 1]:
                return 0
            dict_map[B[i]].pop()
        return 1
