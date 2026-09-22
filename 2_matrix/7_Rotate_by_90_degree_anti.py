class Solution:
    # Function to rotate matrix anticlockwise by 90 degrees.
    def rotateby90(self, a, n):
        # code here
        # take transpose
        for i in range(n):
            for j in range(i, n):
                a[i][j], a[j][i] = a[j][i], a[i][j]

        for i in range((n // 2)):
            for j in range(n):
                a[i][j], a[n - i - 1][j] = a[n - i - 1][j], a[i][j]
        return a
        # 147
        # 258
        # 369

        # 369  741
        # 258  852
        # 147  963
