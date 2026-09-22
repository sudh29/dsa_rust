import math
import datetime

# import numpy
################# ex-1 #######################
print("Hello world")
print("""Hi this is different way""")
print("This is also other way")

############### ex-2 ########################
print()
a = 123
c = 4455.32323
d = 123 + 1333j
print("interger ", a, type(a))
print("float ", c, type(c))
print("complex ", d, type(d))
print("real part", d.real)
print("imaginary part", d.imag)

############### ex-3 ########################
print()
# complex -- float -- int ###################
############## Arthematic ##################
print("addition ", a + c)
print("subtraction ", c - a)
print("Multiplication ", c * a)
print("Division ", c / a)
print("Remainder ", c % a)
print("quotient ", c // a)


############ Builtin function ################
print()
print("power ", pow(2, 10))
print("radians ", math.radians(180))
print("Hexadeciamal ", hex(a))
print("Hex to Deciamal ", 0xA)
## help('modules')
## dir()
## dir(__builtins__)
##


########### Boolean #############
print()
m = 10
n = 10
print("Boolean ", m == n)
print("Boolean ", m > n)
print(int(True))
print(int(False))

########## Date Time ############

my_dob = datetime.date(1992, 8, 29)
print("My date of birth ", my_dob)
print(my_dob.year)
print(my_dob.month)
print(my_dob.day)
message = "I was born on this date {:%A,%B %d,%Y}."
print(message.format(my_dob))
new_date = datetime.date(2000, 1, 1)
dt = datetime.timedelta(100)
print(new_date + dt)

l_date = datetime.date(2000, 1, 12)
l_time = datetime.time(22, 20, 3)
l_datetime = datetime.datetime(2000, 1, 12, 22, 20, 3)
print(l_date)
print(l_time)
print(l_datetime)
print(datetime.datetime.today())
str_time = "7/20/1969"
print("String to date", datetime.datetime.strptime(str_time, "%m/%d/%Y"))
