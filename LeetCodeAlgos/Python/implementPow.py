# Implement pow(x, n), which calculates x raised to the power n (i.e., xn).

class Solution:
    def myPow(self, x: float, n: int) -> float:
        return x ** n
    
s = Solution()
print(s.myPow(2.0, 10))
assert s.myPow(2.0, 10) == 1024.0
print(s.myPow(2.1, 3))
assert round(s.myPow( 2.1, 3 ), 5) == 9.261
print(s.myPow(2.0, -2))
assert s.myPow(2.0, -2) == 0.25