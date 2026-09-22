class Solution:
    def sortedMatrix(self, N, Mat):
        # code here
        temp = []
        for i in Mat:
            temp.extend(i)
        temp.sort()
        for i in range(N):
            for j in range(N):
                Mat[i][j] = temp[i * N + j]
        return Mat
