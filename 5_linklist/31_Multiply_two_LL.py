def multiplyTwoList(head1, head2):
    first = head1
    # first_str=""
    a = 0
    b = 0
    while first:
        # first_str+=str(first.data)
        a = (a * 10 + first.data) % MOD
        first = first.next
    second = head2
    # second_str=""
    while second:
        # second_str+=str(second.data)
        b = (b * 10 + second.data) % MOD
        second = second.next
    # res = ((int(first_str)%MOD) * (int(second_str)%MOD))%MOD
    res = (a * b) % MOD
    return res
