# Given a string s of '(' , ')' and lowercase English characters.
# Your task is to remove the minimum number of parentheses ( '(' or ')', in any positions ) so that the resulting parentheses string is valid and return any valid string.
# Formally, a parentheses string is valid if and only if:
    # It is the empty string, contains only lowercase characters, or
    # It can be written as AB (A concatenated with B), where A and B are valid strings, or
    # It can be written as (A), where A is a valid string.

class Solution:
    def minRemoveToMakeValid(self, s: str) -> str:
        pStack = []
        i = 0
        while i < len(s):
            if s[i] == '(':
                pStack.append(i)
            if s[i] == ')':
                if not pStack:
                    s = s[:i]+s[i+1:]
                    i-=1
                else:
                    pStack.pop()
            i+=1
        for x in range(len(pStack)-1, -1, -1):
            i = pStack[x]
            s = s[:i]+s[i+1:] if len(s) > 1 else ""
        return s

s = Solution()
print(s.minRemoveToMakeValid("lee(t(c)o)de)"))
print(s.minRemoveToMakeValid("a)b(c)d"))
print(s.minRemoveToMakeValid("))(("))
print(s.minRemoveToMakeValid("((((("))