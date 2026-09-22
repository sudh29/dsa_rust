def validShuffle(str1, str2, shuffle):
    n1 = len(str1)
    n2 = len(str2)
    n = len(shuffle)
    if n != n1 + n2:
        return False

    freq = dict()
    for i in range(n1):
        freq[str1[i]] = freq.get(str1[i], 0) + 1
    for i in range(n2):
        freq[str2[i]] = freq.get(str2[i], 0) + 1

    for i in range(n):
        if shuffle[i] in freq:
            freq[shuffle[i]] -= 1
        else:
            return False
    for key, value in freq.items():
        if value != 0:
            return False
    return True


str1 = "BA"
str2 = "XY"
shuffle = "ABYX"

if validShuffle(str1, str2, shuffle):
    print("YES")
else:
    print("NO")
