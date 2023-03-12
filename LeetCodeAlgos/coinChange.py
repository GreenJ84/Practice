# You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.
# Return the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.
# You may assume that you have an infinite number of each kind of coin.

from typing import List
from functools import lru_cache

class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        n = len(coins)
        
        @lru_cache(None)
        def dps(idx, amount):
            if amount == 0:
                return 0
            # Return -1 for invalid searches
            if idx >= n or amount < 0:
                return -1
            # Set a minumim coins needed
            min_coins = float('inf')
            # Search from 0 to the largest possible amount of coins possible at this index
            for x in range(0, amount // coins[idx] + 1):
                # Get the optimal amount of coins for the next idx
                next = dps(idx + 1, amount-x*coins[idx])
                # Make sure the next ind was valid
                if next != -1:
                    # Choose the optimal of current 
                    min_coins = min(min_coins, next+x)
            # If the current coins werent able to fit, return -1
            return -1 if min_coins == float('inf') else min_coins
        # Start searching from the fisrt coin
        return dps(0, amount)

# class Solution:
#     def coinChange(self, coins: List[int], amount: int) -> int:
#         coins.sort()
#         ans = 0
#         for i in range(len(coins)-1, -1, -1):
#             while coins[i] <= amount:
#                 ans += 1
#                 amount -= coins[i]

#         return ans if amount == 0 else -1
    
s = Solution()
print(s.coinChange([1,2,5], 11))
print(s.coinChange([2], 3))
print(s.coinChange([1], 0))
print(s.coinChange([2,5,10,1], 27))
print(s.coinChange([186,419,83,408]
, 6249))
