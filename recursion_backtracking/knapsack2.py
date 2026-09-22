# using greedy programing

# fractional knapsack


def knapsackFractional(maxWeight, weight, value, n):
    total = 0
    ratioDict = {}
    for i in range(n):
        ratioDict[value[i] // weight[i]] = [weight[i], value[i]]

    r = sorted(ratioDict.items(), reverse=True)

    # print(r)

    for i in r:
        currWeight = i[1][0]
        currvalue = i[1][1]

        if maxWeight - currWeight >= 0:
            maxWeight -= currWeight
            total += currvalue
        else:
            fraction = maxWeight / currWeight
            total += currvalue * fraction
            maxWeight = int(maxWeight - currWeight * fraction)
            break
    return int(total)


val = [60, 100, 120]
weight = [10, 20, 30]
w = 50
n = len(val)
print(knapsackFractional(w, weight, val, n))
