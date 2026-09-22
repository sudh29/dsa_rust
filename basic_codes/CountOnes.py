def countone(n):
    res = []
    for i in range(n + 1):
        binVal = bin(i)[2:]
        res.append(binVal.count("1"))
    return res


def countone2(n):
    res = [0]
    for i in range(1, n + 1):
        res.append(res[i // 2] + i % 2)
    return res


N = 5

print(countone(N))
print(countone2(N))
