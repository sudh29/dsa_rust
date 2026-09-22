from typing import List


class Solution:
    def buyMaximumProducts(self, n: int, k: int, price: List[int]) -> int:
        data = [(price[i], i + 1) for i in range(n)]
        data.sort()
        res = 0
        for p, qty in data:
            if p * qty <= k:
                res += qty
                k -= p * qty
            else:
                res += k // p
                # k -= p * (k // p)
                break
        return res


# {
# Driver Code Starts


class IntArray:
    def __init__(self) -> None:
        pass

    def Input(self, n):
        arr = [int(i) for i in input().strip().split()]  # array input
        return arr

    def Print(self, arr):
        for i in arr:
            print(i, end=" ")
        print()


if __name__ == "__main__":
    t = int(input())
    for _ in range(t):
        n, k = map(int, input().strip().split())

        price = IntArray().Input(n)

        obj = Solution()
        res = obj.buyMaximumProducts(n, k, price)

        print(res)
