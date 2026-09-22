def printDups(Str):
    count = {}
    for i in range(len(Str)):
        if Str[i] in count:
            count[Str[i]] += 1
        else:
            count[Str[i]] = 1
    temp = []
    for it, it2 in count.items():
        if it2 > 1:
            temp.append(it)
    return temp


Str = "test string"
printDups(Str)
