#!/usr/bin/python
import os

# Open a file
fo = open("foo.txt", "w+")
print("Name of the file: ", fo.name)
print("Closed or not : ", fo.closed)
print("Opening mode : ", fo.mode)
# print("Softspace flag : ", fo.softspace)

fo.write("Python is a great language.\nYeah its great!!\n")
str = fo.read(10)
print("Read String is : ", str)

position = fo.tell()
print("Current file position : ", position)
# Reposition pointer at the beginning once again
position = fo.seek(0, 0)
str = fo.read(10)
print("Again read String is : ", str)

# Rename a file from test1.txt to test2.txt
os.rename("foo.txt", "test2.txt")
# Delete file test2.txt
# os.remove("text2.txt")
# Close opend file
fo.close()
