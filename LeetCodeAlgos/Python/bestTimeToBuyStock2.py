# You are given an integer array prices where prices[i] is the price of a given stock on the ith day.
# On each day, you may decide to buy and/or sell the stock. You can only hold at most one share of the stock at any time. However, you can buy it then immediately sell it on the same day.
# Find and return the maximum profit you can achieve.

from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        profit = 0
        stock = prices[0]

        for i in range(1, len(prices)):
            if prices[i] < prices[i - 1] and prices[i - 1] > stock:
                profit += prices[i - 1] - stock
                stock = prices[i]
            elif prices[i] < stock:
                stock = prices[i]
            elif i == len(prices) - 1 and prices[i] > stock:
                profit += prices[i] - stock

        return profit

s = Solution();
print(s.maxProfit([7,1,5,3,6,4]))
assert s.maxProfit([7,1,5,3,6,4]) == 7

print(s.maxProfit([1,2,3,4,5]))
assert s.maxProfit([1,2,3,4,5]) == 4

print(s.maxProfit([7,6,4,3,1]))
assert s.maxProfit([7,6,4,3,1]) == 0