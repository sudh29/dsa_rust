############### If then else ###################
ip = "Enter the string  "
if len(ip) < 6:
    print("true")
else:
    print("False")


############### Functions ##################
def f():
    pass
    return print("Pass")


f()

############### Sets ####################
new_set = set()
new_set.add(42)
new_set.add("true")
new_set.add(133.23)
new_set.add(42)
new_set.add(42)
print(new_set, len(new_set))
# new_set.remove(42)
new_set.discard(12)
# union
# intersection
# 4 in set --- true or false

############ Lists ################
print()
primes = [2, 3, 5, 7, 11, 13]
primes.append(17)
primes.append(19)
print("List ", primes)
print(primes[0])
print(primes[-1])
print("Slicing ", primes[2:5])
primes.reverse()
print(primes)

############ Dictionaries #############
print()
new_dict = {
    "user_id": 12,
    "message": "Hey world",
    "language": "English",
    "datetime": "2020/12/13",
}
print(new_dict)
try:
    print(new_dict["user_id"])
except KeyError:
    print("Error ")

for key in new_dict.keys():
    value = new_dict[key]
    print(key, "=", value)

# pop popitem clear

############## Tuples less m/m than list ###################
# cannot be changed
print()
tup_new = (1, 2, 3, 4, 4, 5, 6)
print("Tuples ", tup_new)
import timeit

list_test = timeit.timeit(stmt="[1,1,2,3,4]", number=100000)
tuple_test = timeit.timeit(stmt="[1,1,2,3,4]", number=100000)
print("List time : ", list_test)
print("Tuple time : ", tuple_test)

################## Logging ############################
# Levels : Debug 10 , Info 20 , Warning 30 , Error 40, Critical 50
import logging

# create and configure logger
# LOG_FORMAT = "%(Levelname)s %(asctime)s - %(message)s"
logging.basicConfig(
    filename="/home/sudhanshu/Desktop/Python/work/logfile.log",
    level=logging.DEBUG,
    # format=LOG_FORMAT,
    filemode="w",
)
logger = logging.getLogger()
# Test the logger
logger.debug("Hey this is a log file")
logger.info("Hey this is a log file")
logger.warning("Hey this is a log file")
logger.error("Hey this is a log file")
logger.critical("Hey this is a log file")


################## Recursion ############################
# Fibonacci Sequence
from functools import lru_cache


# also can use
@lru_cache(maxsize=1000)
# memoization
# fib_cache = {}
def fib_rec_fn(n):
    global value
    # if n in fib_cache:
    #     return fib_cache[n]
    if n == 1:
        value = 1
    elif n == 2:
        value = 1
    elif n > 2:
        value = fib_rec_fn(n - 1) + fib_rec_fn(n - 2)
    # fib_cache[n] = value
    return value


for n in range(11):
    print(n, " : ", fib_rec_fn(n))
