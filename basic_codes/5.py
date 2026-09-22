################ File ######################
# Text File
# Binary File
f = open("/home/sudhanshu/Desktop/Python/work/file.txt")
data = f.read()
f.close()
print(data)
print()
# Another method no need to close the file
try:
    with open("/home/sudhanshu/Desktop/Python/work/fille.txt") as fpt:
        fdata = fpt.read()
except FileNotFoundError:
    fdata = None

print(fdata)

oceans = ["a", "b", "c", "d"]
with open("/home/sudhanshu/Desktop/Python/work/file1.txt", "w") as fpt1:
    for oceans in oceans:
        print(oceans, file=fpt1)


############### Unit Tests ###################
