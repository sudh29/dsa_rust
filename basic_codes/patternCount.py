import os

os.system("clear")


def pattern_check(a, b):
    len_a = len(a)
    len_b = len(b)
    temp_list = []
    i = 0
    count = 0
    while i < len_b:
        for j in range(len_a):
            temp_list.append(b[j + i])
        # print(i,"temp -- ",temp_list)
        i = i + len_a
        if temp_list == a:
            count = count + 1
        temp_list.clear()
    return print("Non-overlapping count: ", count)


def pattern_check_over(a, b):
    len_a = len(a)
    len_b = len(b)
    temp_list = []
    i = 0
    count = 0
    while i < len_b:
        for j in range(len_a):
            if i + j < len_b:
                temp_list.append(b[j + i])
        # print(i,"temp -- ",temp_list)
        i = i + 1
        if temp_list == a:
            count = count + 1
        temp_list.clear()
    return print("Overlapping count: ", count)


list1 = [1, 1, 1]
list2 = [1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0]
pattern_check_over(list1, list2)
pattern_check(list1, list2)
