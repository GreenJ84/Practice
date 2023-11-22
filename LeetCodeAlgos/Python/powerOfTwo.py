# Given an integer n, return true if it is a power of two. Otherwise, return false.
# An integer n is a power of two, if there exists an integer x such that n == 2x.

class Solution:
    def isPowerOfTwo(self, n: int) -> bool:
        if n < 1:
            return False
        while n > 1:
            m = n >> 1
            if m << 1 != n:
                return False
            n = m
        return True

s = Solution()
s.isPowerOfTwo(1)
s.isPowerOfTwo(16)
s.isPowerOfTwo(3)