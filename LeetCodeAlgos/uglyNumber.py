# An ugly number is a positive integer whose prime factors are limited to 2, 3, and 5.
# Given an integer n, return true if n is an ugly number.

class Solution:
    def isUgly(self, n: int) -> bool:
        if n <= 0:
            return False
        elif n in [1,2,3,5]:
            return True
        if n%2==0:
            while(n%2==0):
                n/=2
        if n%3==0:
            while(n%3==0):
                n/=3
        if n%5==0:
            while(n%5==0):
                n/=5
        return True if n == 1 else False

s = Solution()
print(s.isUgly(6))
print(s.isUgly(1))
print(s.isUgly(14))