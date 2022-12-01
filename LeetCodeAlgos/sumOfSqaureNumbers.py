# Given a non-negative integer c, decide whether there're two integers a and b such that a2 + b2 = c.

class Solution:
    def judgeSquareSum(self, c: int) -> bool:
        start, end = 0, int(c**.5)
        while start<=end:
            x, y = start**2, end**2
            if x + y == c:
                return True
            elif x + y > c:
                end-=1
            else:
                start+=1
        return False

s = Solution()
print(s.judgeSquareSum(5))
print(s.judgeSquareSum(3))
print(s.judgeSquareSum(2))

