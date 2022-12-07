# Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.
#! Note: You must not use any built-in BigInteger library or convert the inputs to integer directly.

class Solution:
    def multiply(self, num1: str, num2: str) -> str:
        if num1=="0" or num2=="0":
            return "0"
        temp, prod = [], 0

        for i in range(-1, -len(num2)-1, -1):
            for j in range(-1, -len(num1)-1, -1):
                prod = int(num2[i]) * int(num1[j]) + prod

                if abs(i) == 1 or -j-i-2>=len(temp):
                    temp.append(prod%10)

                else:
                    prod+=temp[-j-i-2]
                    temp[-j-i-2] = prod%10

                prod=prod//10
            if prod != 0: 
                temp.append(prod)
                prod=prod//10
        res = ''
        for i in range(len(temp)-1, -1, -1):
            res+=str(temp[i])
        return res

s = Solution()
# print(s.multiply('2', '3'))
# print(s.multiply('9', '9'))
print(s.multiply('999', '999'))
# print(s.multiply('123', '456'))