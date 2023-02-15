# Given two non-negative integers, num1 and num2 represented as string, return the sum of num1 and num2 as a string.
# You must solve the problem without using any built-in library for handling large integers (such as BigInteger). You must also not convert the inputs to integers directly.

class Solution:
    def addStrings(self, num1: str, num2: str) -> str:
        res = ""
        temp = 0
        for i in range(1, max(len(num1), len(num2))+1):
            temp += int(num1[-i]) if i <= len(num1) else 0
            temp += int(num2[-i]) if i <= len(num2) else 0
            res = str(temp % 10) + res
            temp = temp//10
        while temp:
            res = str(temp % 10)+res
            temp = temp//10
        return res

s = Solution()
print(s.addStrings("11","123"))
print(s.addStrings("456","77"))
print(s.addStrings("0","0"))