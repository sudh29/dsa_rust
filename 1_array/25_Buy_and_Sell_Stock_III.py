class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        n = len(prices)
        left = [0 for i in range(n)]
        right = [0 for i in range(n)]
        min_left = prices[0]
        max_left_profit = 0
        max_right = prices[-1]
        max_right_profit = 0
        for i in range(1, len(prices)):
            profit = prices[i] - min_left
            max_left_profit = max(max_left_profit, profit)
            min_left = min(min_left, prices[i])
            left[i] = max_left_profit
        print(left)
        for i in range(len(prices) - 2, -1, -1):
            profit = max_right - prices[i]
            max_right_profit = max(max_right_profit, profit)
            max_right = max(max_right, prices[i])
            right[i] = max_right_profit
        print(right)

        res = right[0]
        for i in range(len(left) - 1):
            profit = left[i] + right[i + 1]
            res = max(res, profit)
        return res
