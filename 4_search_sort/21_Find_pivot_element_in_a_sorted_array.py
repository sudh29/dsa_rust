def get_pivot_element(array, left, right):
    if right < left:
        return -1
    if right == left:
        return left

    middle = (left + right) // 2
    if middle < right and array[middle] > array[middle + 1]:
        return middle
    if middle > left and array[middle] < array[middle - 1]:
        return middle - 1

    if array[left] >= array[middle]:
        return get_pivot_element(array, left, middle - 1)
    else:
        return get_pivot_element(array, middle + 1, right)


# Example usage:
array = [4, 5, 6, 7, 8, 1, 2, 3]
pivot_index = get_pivot_element(array, 0, len(array) - 1)
print("Pivot Element Index:", pivot_index)
print("Pivot Element:", array[pivot_index] if pivot_index != -1 else "No Pivot")
