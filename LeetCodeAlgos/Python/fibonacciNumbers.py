# The Fibonacci numbers, commonly denoted F(n) form a sequence, called the Fibonacci sequence, such that each number is the sum of the two preceding ones, starting from 0 and 1. 

class Solution:
    def fib(self, n: int) -> int:
        match n:
            case 0:
                return 0
            case 1:
                return 1
            case _:
                f = 0
                l = 1
                sum = 0
                for i in range(1, n):
                    sum = f + l
                    f = l
                    l = sum
                return sum

s = Solution()
print(s.fib(1))
print(s.fib(3))
print(s.fib(5))