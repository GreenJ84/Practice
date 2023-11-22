# You have n coins and you want to build a staircase with these coins. The staircase consists of k rows where the ith row has exactly i coins. The last row of the staircase may be incomplete.
# Given the integer n, return the number of complete rows of the staircase you will build.

class Solution:
    def arrangeCoins(self, n: int) -> int:
        step, res = 1, 0
        while n >= step:
            res+=1
            n-=step
            step+=1
        return res

s = Solution()
print(s.arrangeCoins(5))
print(s.arrangeCoins(8))