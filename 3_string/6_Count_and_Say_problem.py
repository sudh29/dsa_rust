def rel(ip: str) -> str:
    res = []
    c = 1
    char = ip[0]
    for i in range(1, len(ip)):
        if char == ip[i]:
            c += 1
        else:
            res.append(f"{c}{char}")
            char = ip[i]
            c = 1
    res.append(f"{c}{char}")
    return "".join(res)


class Solution:
    def countAndSay(self, n: int) -> str:
        val = "1"
        for _ in range(n - 1):
            val = rel(val)
        return val
