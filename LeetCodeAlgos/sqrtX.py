# Given a non-negative integer x, return the square root of x rounded down to the nearest integer. The returned integer should be non-negative as well.
# You must not use any built-in exponent function or operator.

class Solution:
    def mySqrt(self, x: int) -> int:
        start, end = 0, x
        while start<=end:
            m=int((start+end)/2)
            if m*m==x:
                return m
            elif m*m>x:
                if (m-1)*(m-1)<x:
                    return m-1
                end=m-1
            else:
                if (m+1)*(m+1)>x:
                    return m
                start=m+1

s = Solution()
print(s.mySqrt(4))
print(s.mySqrt(8))
print(s.mySqrt(9))
print(s.mySqrt(16))