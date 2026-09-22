# using dynamic programing


def knapsack(maxWeight, weight, value, n):
    k = [[0 for _ in range(maxWeight + 1)] for _ in range(n + 1)]

    for i in range(n + 1):
        for j in range(maxWeight + 1):
            if i == 0 or j == 0:
                k[i][j] = 0
            elif weight[i - 1] <= w:
                k[i][j] = max(val[i - 1] + k[i - 1][j - weight[i - 1]], k[i - 1][j])
            else:
                k[i][j] = k[i - 1][j]
    # print(k)
    return k[n][maxWeight]


val = [60, 100, 120]
weight = [10, 20, 30]
w = 50
n = len(val)
print(knapsack(w, weight, val, n))
