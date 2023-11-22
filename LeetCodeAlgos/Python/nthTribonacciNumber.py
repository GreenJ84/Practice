# The Tribonacci sequence Tn is defined as follows: 
# T0 = 0, T1 = 1, T2 = 1, and Tn+3 = Tn + Tn+1 + Tn+2 for n >= 0.
# Given n, return the value of Tn.

class Solution:
    def tribonacci(self, n: int) -> int:
        match n:
            case 0:
                return 0
            case 1 | 2:
                return 1
            case _:
                sum, x, y, z = 0, 0, 1, 1
                for i in range(2, n):
                    sum = x + y + z
                    x, y, z = y, z, sum
                return sum

s = Solution()
print(s.tribonacci(1))
# print(s.tribonacci(3))
# print(s.tribonacci(4))
# print(s.tribonacci(25))