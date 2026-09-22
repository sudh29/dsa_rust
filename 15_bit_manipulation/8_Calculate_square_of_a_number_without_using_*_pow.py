def square(n):
    if n < 0:
        n = -n
    res = n
    for i in range(1, n):
        res += n
    return res


def square(n):
    if n == 0:
        return 0
    if n < 0:
        n = -n

    x = n >> 1
    if n & 1:
        return (square(x) << 2) + (x << 2) + 1

    else:
        return square(x) << 2
