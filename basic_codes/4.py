############## List comp ################
list1 = [i**2 for i in range(11)]
print(list1)

A = [1, 3, 5, 7]
B = [2, 4, 6, 8]
c_prod = [(a, b) for a in A for b in B if a > 3 if b > 4]
print(c_prod)

movies = ["acsc", "bsds", "vscsd", "gscs", "gasxc", "jcsdc"]
gmovie = [title for title in movies if title.startswith("g")]
print(gmovie)

################## Classes ##########################
import datetime


class USER:
    pass


user1 = USER()
user1.first_name = "Dave"
user1.last_name = "Bowan"
print(user1.first_name, user1.last_name)


class User:
    def __init__(self, full_name, dob):
        self.name = full_name
        self.birthday = dob  # yyyymmdd
        name_pieces = full_name.split(" ")
        self.f_name = name_pieces[0]
        self.l_name = name_pieces[-1]

    def age(self):
        today = datetime.date(2020, 3, 8)
        yyyy = int(self.birthday[0:4])
        mm = int(self.birthday[4:6])
        dd = int(self.birthday[6:8])
        dob1 = datetime.date(yyyy, mm, dd)
        age_days = (today - dob1).days
        age_yr = age_days / 365
        return age_yr


user0 = User("Sudhanshu Chaudhary", "19930829")
print(user0.name, user0.birthday)
print(user0.f_name)
print(user0.l_name)
print(user0.age())


################## Pydoc #####################


############## Jason #######################


############## Lambda  ###############

g = lambda x: 3 * x + 1
print(g(2))
full_name = lambda fn, ln: fn.strip().title() + " " + ln.strip().title()
print(full_name("     Sudhanshu   ", "   Chaudhary  "))


def q_fn(a, b, c):
    return lambda x: a * x**2 + b * x + c


f1 = q_fn(2, 3, -5)

print("Q_equation value: ", f1(5))


################# Map Filter Reduce List #####################
import math


def area_circle(r):
    area_c = math.pi * (r**2)
    return area_c


list_radii = [1, 2, 3, 4, 5, 6]
area = []
print("Area of cicle : ", list(map(area_circle, list_radii)))

# filter
import statistics

data = [1.3, 4.3, 5.1, 9.8, 7.8]
avg = statistics.mean(data)
print(avg)
print(list(filter(lambda x: x > avg, data)))

# Filter the null values
countries = ["", "India", "", "", "USA"]
print(list(filter(None, countries)))

# Reduce fn is used in python2 in python3 functools is used or for loop
from functools import reduce

# multiply all number in a list
multi = lambda x, y: x * y
print(reduce(multi, data))


################### Sorting #########################

movies.sort()
print("Sorted data: ", movies)
movies.sort(reverse=True)
print("Sorted data: ", movies)

# size= lambda planet : planet[1]
# data.sort(key=size,reverse=True)

# sort copy
sorted_movies = sorted(movies)
print("Sorted copy :", sorted_movies)
print(sorted("Sudhanshu"))
