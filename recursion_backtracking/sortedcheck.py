# sorted array check using recursion


def check(A):
    if len(A) == 1:
        return True
    return A[0] <= A[1] and check(A[1:])


A = [1, 2, 3, 4, 5, 6, 7]

print(check(A))
print()
B = [1, 5, 671, 1, 6, 3, 2, 0]
print(check(B))
print()
