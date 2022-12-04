# Evaluate the value of an arithmetic expression in Reverse Polish Notation.
# Valid operators are +, -, *, and /. Each operand may be an integer or another expression.
# Note that division between two integers should truncate toward zero.
# It is guaranteed that the given RPN expression is always valid. That means the expression would always evaluate to a result, and there will not be any division by zero operation.

from typing import List


class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        stack=[]
        
        for n in tokens:
            if n.isdigit() or len(n)>1:
                stack.append(int(n))
            else:
                stack.append(update(n, stack))
        return stack.pop()

def update(sign, stack):
    n2,n1=stack.pop(),stack.pop()
    if sign=="+": return n1+n2
    if sign=="-": return n1-n2
    if sign=="*": return n1*n2
    if sign=="/": return int(n1/n2)

s = Solution()
print(s.evalRPN(["2","1","+","3","*"]))
print(s.evalRPN(["4","13","5","/","+"]))
print(s.evalRPN(["10","6","9","3","+","-11","*","/","*","17","+","5","+"]))