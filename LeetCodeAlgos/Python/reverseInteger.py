# Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.
# Assume the environment does not allow you to store 64-bit integers (signed or unsigned).

class Solution:
    def reverse(self, x: int) -> int:
        x = str(x)
        res = ''
        for i in range(len(x)-1, -1, -1):
            if i == 0 and x[i] == '-':
                res = x[i]+res
            else:
                res+=x[i]
        res = int(res)
        return res if 2147483647>res and res> -2147483647 else 0

s = Solution()
print(s.reverse(123))
print(s.reverse(-123))
print(s.reverse(120))