# A self-dividing number is a number that is divisible by every digit it contains.

# For example, 128 is a self-dividing number because 128 % 1 == 0, 128 % 2 == 0, and 128 % 8 == 0.
# A self-dividing number is not allowed to contain the digit zero.

# Given two integers left and right, return a list of all the self-dividing numbers in the range [left, right].

from typing import List


class Solution:
    def selfDividingNumbers(self, left: int, right: int) -> List[int]:
        ans = []

        def checkDiv(num):
            check = str(num)
            if check[0] * len(check) == check: 
                return True
            for i in check:
                if i == '0' or num % int(i) != 0:
                    return False
            return True

        for i in range(left, right + 1):
            if i < 10 or checkDiv(i):
                ans.append(i)
        return ans

s = Solution()
print(s.selfDividingNumbers(1, 22))
print(s.selfDividingNumbers(47, 85))