def getMinMax(a, n):
    min_val = float("inf")
    max_val = float("-inf")
    for i in a:
        if i > max_val:
            max_val = i
        if i < min_val:
            min_val = i
    return [min_val, max_val]


# {
# Driver Code Starts
# Initial Template for Python 3


def main():
    T = int(input())

    while T > 0:
        n = int(input())
        a = [int(x) for x in input().strip().split()]

        product = getMinMax(a, n)
        print(product[0], end=" ")
        print(product[1])

        T -= 1


if __name__ == "__main__":
    main()
