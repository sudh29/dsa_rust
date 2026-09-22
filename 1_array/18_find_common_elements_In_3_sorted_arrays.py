class Solution:
    def commonElements(self, A, B, C, n1, n2, n3):
        # Extra space
        temp = {}
        A = list(set(A))
        B = list(set(B))
        C = list(set(C))
        for i in A:
            temp[i] = temp.get(i, 0) + 1
        for i in B:
            temp[i] = temp.get(i, 0) + 1
        for i in C:
            temp[i] = temp.get(i, 0) + 1
        res = []
        temp1 = temp
        val = sorted(temp)
        for i in val:
            if temp1[i] == 3:
                res.append(i)
        return res

        # # 3 Pointer
        # i, j, k = 0, 0, 0
        # common_elements = set()
        # while i < n1 and j < n2 and k < n3:
        #     if A[i] == B[j] == C[k]:
        #         common_elements.add(A[i])
        #         i += 1
        #         j += 1
        #         k += 1
        #     elif A[i] < B[j]:
        #         i += 1
        #     elif B[j] < C[k]:
        #         j += 1
        #     else:
        #         k += 1
        # return sorted(list(common_elements))
