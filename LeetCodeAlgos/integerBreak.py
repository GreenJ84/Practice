# Given an integer n, break it into the sum of k positive integers, where k >= 2, and maximize the product of those integers.
# Return the maximum product you can get.


## AM-GM Inequality (Arithmetic mean >= Geometric mean)
""" 
    The product, P, of K integers whose sum is S, is maximized when
    those K integers are as close to equal as possible
""" 
import math

class Solution:
    def integerBreak(self, n: int) -> int:
        ans = 0
        k = 2
        while k <= 10 and k <= n:
            # Get mean for k values
            num = round(n / k)
            # Get last offset value
            if num*k <= n:
                last = num + n % k 
            else:
                last = num - (num*k - n)
            front = num**(k-1)
            ans = max(ans, (front*last))
            k += 1
        return ans




s = Solution()
# print(s.integerBreak(2))
# print(s.integerBreak(10))
# print(s.integerBreak(8))
print(s.integerBreak(32))