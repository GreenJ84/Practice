# Given an integer n, return true if n has exactly three positive divisors. Otherwise, return false.

# An integer m is a divisor of n if there exists an integer k such that n = k * m.

class Solution:
    def isThree(self, n: int) -> bool:
        # Not square not possible
        if n < 4 or n ** .5 % 1 != 0:
            return False
        for i in range(2, int(n ** .5)):
            if n % i == 0:
                return False
        return True