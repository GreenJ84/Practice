# It is a sweltering summer day, and a boy wants to buy some ice cream bars.
# At the store, there are n ice cream bars. You are given an array costs of length n, where costs[i] is the price of the ith ice cream bar in coins. The boy initially has coins coins to spend, and he wants to buy as many ice cream bars as possible. 
# Return the maximum number of ice cream bars the boy can buy with coins coins.
# Note: The boy can buy the ice cream bars in any order.

from typing import List


class Solution:
    def maxIceCream(self, costs: List[int], coins: int) -> int:
        costs.sort()

        bars = 0;
        for i in costs:
            if i>coins:
                return bars
            coins-=i
            bars+=1
        return bars

s = Solution()
print(s.maxIceCream([1,3,2,4,1],7))
print(s.maxIceCream([10,6,8,7,7,8],5))
print(s.maxIceCream([1,6,3,1,2,5],20))
