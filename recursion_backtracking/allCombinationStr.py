def allCombinationStr(arr, prefix, n, k, res):
    if k == 0:
        res.append(prefix)
        # print(prefix)
        return
    for i in range(n):
        newPrefix = prefix + arr[i]
        allCombinationStr(arr, newPrefix, n, k - 1, res)


if __name__ == "__main__":
    print("First Test")
    set1 = ["1", "2", "3"]
    k = 2
    res = []
    allCombinationStr(set1, "", len(set1), k, res)
    print(res)
    print()
    print("\nSecond Test")
    set2 = ["a", "b", "c", "d"]
    k = 4
    res2 = []
    allCombinationStr(set2, "", len(set2), k, res2)
    print(res2)
    print()
