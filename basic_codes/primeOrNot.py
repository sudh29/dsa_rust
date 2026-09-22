############### Prime numbers #####################
import math
import time


def prime_v1(n):
    """this is function for prime"""
    if n == 1:
        return False
    if n == 2:
        return True
    if n > 2 and n % 2 == 0:
        return False
    max_divisor = math.floor(math.sqrt(n))
    for i in range(3, max_divisor + 1, 2):
        if n % i == 0:
            return False
    return True


for n in range(1, 5):
    print(n, prime_v1(n))

# calculate the time for generating primes
t0 = time.time()
for n in range(1, 100000):
    prime_v1(n)
t1 = time.time()
print("Time required : ", t1 - t0)
