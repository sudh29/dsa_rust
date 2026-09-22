# Tower of Hanoi using recursion


def th(n, fr, to, by):
    if n == 1:
        print("Move disk 1 from", fr, "to", to)
        return
    th(n - 1, fr, by, to)
    print("Move disk", n, "from", fr, "to", to)
    th(n - 1, by, to, fr)


th(3, "A", "B", "C")
