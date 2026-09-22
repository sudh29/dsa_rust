def reverse(S):
    # temp=[]
    # for i in S:
    #     temp.insert(0,i)
    # val=''.join(temp)
    # return val

    temp = list(S)
    res = ""
    for i in range(len(temp)):
        res += temp.pop()
    return res
