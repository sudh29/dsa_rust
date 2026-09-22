def mergeArrays(arr1, arr2, n1, n2):
    arr3 = []
    i, j, k = 0, 0, 0

    # Merge elements from both arrays until one of them is exhausted
    while i < n1 and j < n2:
        if arr1[i] < arr2[j]:
            arr3.append(arr1[i])
            i += 1
        else:
            arr3.append(arr2[j])
            j += 1

    # If any elements left in arr1, append them to arr3
    while i < n1:
        arr3.append(arr1[i])
        i += 1

    # If any elements left in arr2, append them to arr3
    while j < n2:
        arr3.append(arr2[j])
        j += 1

    return arr3
