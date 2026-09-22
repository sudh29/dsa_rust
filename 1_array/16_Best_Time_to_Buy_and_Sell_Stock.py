class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        n = len(prices)
        max_profit = 0
        min_val = 100000
        for i in range(n):
            min_val = min(min_val, prices[i])
            max_profit = max(max_profit, prices[i] - min_val)
        return max_profit
