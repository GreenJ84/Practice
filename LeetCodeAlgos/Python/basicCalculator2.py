# Given a string s which represents an expression, evaluate this expression and return its value. 
# The integer division should truncate toward zero.
# You may assume that the given expression is always valid. All intermediate results will be in the range of [-231, 231 - 1].
# Note: You are not allowed to use any built-in function which evaluates strings as mathematical expressions, such as eval().

import collections


class Solution:
    def calculate(self, s: str) -> int:
        s += '+'
        pre = "+"
        curr = ""
        stk = []

        for i in s:
            if i == " ":
                continue
            if i in ["+","-","*","/"]:
                if pre == "+":
                    stk.append(int(curr))
                elif pre == "-":
                    stk.append(-int(curr))
                elif pre == "*":
                    stk.append(int(stk.pop() * int(curr)))
                elif pre == "/":
                    stk.append(int(stk.pop() / int(curr)))
                pre = i
                curr = ""
            else:
                curr += i
        return int(sum(stk))



s = Solution()
print(s.calculate("3+2*2"))
print(s.calculate(" 3/2 "))
print(s.calculate("3+5 / 2"))
print(s.calculate("0"))
print(s.calculate("42"))
print(s.calculate("14/3*2"))