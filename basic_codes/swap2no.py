# function to swap two numbers
def swap(a, b):
    a = a ^ b
    b = a ^ b
    a = a ^ b
    return a, b


def swap2(a, b):
    a = a + b
    b = a - b
    a = a - b
    return a, b


def swap3(a, b):
    a, b = b, a
    return a, b


x = 100
y = 50
print("Numbers are: ", x, y)
print("After swapping")
print(swap(x, y))
print(swap2(x, y))
print(swap3(x, y))
