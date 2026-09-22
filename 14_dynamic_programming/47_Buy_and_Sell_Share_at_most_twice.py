from typing import List


class Solution:
    def maxProfit(self, n: int, price: List[int]) -> int:
        if n == 0:
            return 0
        left_profit = [0] * n
        right_profit = [0] * n
        min_price = price[0]
        for i in range(1, n):
            min_price = min(min_price, price[i])
            left_profit[i] = max(left_profit[i - 1], price[i] - min_price)
        max_price = price[n - 1]
        for i in range(n - 2, -1, -1):
            max_price = max(max_price, price[i])
            right_profit[i] = max(right_profit[i + 1], max_price - price[i])
        max_profit = 0
        for i in range(n):
            max_profit = max(max_profit, left_profit[i] + right_profit[i])
        return max_profit


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
