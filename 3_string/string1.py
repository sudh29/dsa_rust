# pattern matching


def sol(a, b):
    if not b:
        return 0
    for i in range(len(a) - len(b) + 1):
        stri = i
        patterni = 0
        while stri < len(a) and patterni < len(b) and a[stri] == b[patterni]:
            stri += 1
            patterni += 1
        if patterni == len(b):
            return i
    return -1


x = "aaaaaabc"
p = "abc"
p2 = "xyz"
print(sol(x, p))
print(sol(x, p2))
