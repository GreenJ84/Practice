# Given an integer number n, return the difference between the product of its digits and the sum of its digits.

class Solution:
    def subtractProductAndSum(self, n: int) -> int:
        sum, prod = 0, 1
        for i in str(n):
            sum += int(i)
            prod *= int(i)
        return prod-sum

s = Solution()
print(s.subtractProductAndSum(234))
print(s.subtractProductAndSum(4421))