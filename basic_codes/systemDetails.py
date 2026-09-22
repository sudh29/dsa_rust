import struct

print(struct.calcsize("P") * 8)
import platform
import os

print(os.name)
print(platform.system())
print(platform.release())

x = 5464312133435443


def sum_d(n):
    sum = 0
    for i in range(len(str(n)) - 1, -1, -1):
        j = n / 10 ** (i)
        sum += int(j)
        k = n % 10 ** (i)
        n = k
    return sum


print(sum_d(x))

import sys

print()
if sys.byteorder == "little":
    # intel, alpha
    print("Little-endian platform.")
else:
    # motorola, sparc
    print("Big-endian platform.")
print()


# to clear terminal after program execution
import os
import time

for i in range(5):
    print("-------------")
print("Clear Terminal")
time.sleep(5)
os.system("clear")
