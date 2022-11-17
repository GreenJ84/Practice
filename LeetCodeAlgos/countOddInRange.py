# Given two non-negative integers low and high. Return the count of odd numbers between low and high (inclusive).

class Solution:
    def countOdds(self, low: int, high: int) -> int:
        if low%2==1 and high%2==1:
            return (high+1-low)//2+1
        else:
            return (high+1-low)//2


s = Solution()
print(s.countOdds(3, 7))
print(s.countOdds(8, 10))
