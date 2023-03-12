# You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.
# Return the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.
# You may assume that you have an infinite number of each kind of coin.

from typing import List


class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        coins.sort()
        ans = 0
        for i in range(len(coins)-1, -1, -1):
            while coins[i] <= amount:
                ans += 1
                amount -= coins[i]

        return ans if amount == 0 else -1
    
s = Solution()
# print(s.coinChange([1,2,5], 11))
# print(s.coinChange([2], 3))
# print(s.coinChange([1], 0))
# print(s.coinChange([2,5,10,1], 27))
print(s.coinChange([186,419,83,408]
, 6249))
