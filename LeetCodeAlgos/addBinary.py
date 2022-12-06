# Given two binary strings a and b, return their sum as a binary string.

class Solution:
    def addBinary(self, a: str, b: str) -> str:
        res, sum = '', 0
        lenn = -max(len(a), len(b))

        for i in range(-1, lenn-1, -1):
            if i >= -len(a):
                sum+=int(a[i])

            if i >= -len(b):
                sum+=int(b[i])

            res+=str(sum%2)
            sum = sum//2

        if sum == 1:
            res+=str(sum)
        res = ''.join(reversed(res))
        return res

s = Solution()
print(s.addBinary("11", "1"))
print(s.addBinary("1010", "1011"))