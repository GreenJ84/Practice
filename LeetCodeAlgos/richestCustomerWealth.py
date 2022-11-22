# You are given an m x n integer grid accounts where accounts[i][j] is the amount of money the i​​​​​​​​​​​th​​​​ customer has in the j​​​​​​​​​​​th​​​​ bank. Return the wealth that the richest customer has.
# A customer's wealth is the amount of money they have in all their bank accounts. The richest customer is the customer that has the maximum wealth.

from typing import List


class Solution:
    def maximumWealth(self, accounts: List[List[int]]) -> int:
        maxx = sum(accounts[0])
        for i in range(1, len(accounts)):
            maxx = max(maxx, sum(accounts[i]))
        return maxx

s = Solution()
print(s.maximumWealth([[1,2,3],[3,2,1]]))
print(s.maximumWealth([[1,5],[7,3],[3,5]]))
print(s.maximumWealth([[2,8,7],[7,1,3],[1,9,5]]))