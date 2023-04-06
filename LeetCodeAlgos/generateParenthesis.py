# Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

from typing import List

class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        ans = []
        if n == 0: return ans

        def backtrack(s: str, open: int, close: int):
            if len(s) == 2*n:
                ans.append(s)
                return
            if open < n:
                backtrack(s+'(', open + 1, close)
            if close < open:
                backtrack(s+')', open, close+1)

        backtrack("", 0, 0)
        return ans

## First brute force solution
# class Solution:
#     def generateParenthesis(self, n: int) -> List[str]:
#         ans = []
#         comp = n*2

#         if n == 0: return ans

#         def addParens(rang, curr, diff):
#             stk = []
#             for _ in range(rang):
#                 curr += "("
#                 stk.append(")")
#             while stk:
#                 curr += stk.pop()

#             if len(curr) == n*2:
#                 ans.append(curr)
#             else:
#                 x = comp-len(curr)
#                 rang = x//2
#                 for i in range(rang):
#                     addParens(i+1, curr[:])
            
#         for i in range(n):
#             addParens(i+1, "", 0)
#         return ans

s = Solution()
print(s.generateParenthesis(3))
print(s.generateParenthesis(2))
print(s.generateParenthesis(1))
print(s.generateParenthesis(0))