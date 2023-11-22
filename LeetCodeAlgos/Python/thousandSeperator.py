# Given an integer n, add a dot (".") as the thousands separator and return it in string format.

class Solution:
    def thousandSeparator(self, n: int) -> str:
        if n < 1000: 
            return str(n)
        ans = ''
        while n > 0:
            if len(ans) % 4 == 3:
                ans = '.' + ans
            ans = str(n % 10) + ans
            n = n // 10
        return ans