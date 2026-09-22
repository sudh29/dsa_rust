# function for anagram or not
def anagrams(a, b):
    if len(a) != len(b):
        return 0
    a.lower()
    b.lower()
    if sorted(a) == sorted(b):
        return 1
    else:
        return 0


x = "sudh"
y = "Rama"
print(anagrams(x, y))

i = "dance"
j = "cadne"
print(anagrams(i, j))
