# Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
# An input string is valid if:
    # Open brackets must be closed by the same type of brackets.
    # Open brackets must be closed in the correct order.
    # Every close bracket has a corresponding open bracket of the same type.

class Solution:
    def isValid(self, s: str) -> bool:
        res = []
        for i in s:
            match i:
                case '(' | '[' | '{':
                    res.append(i)
                case ')':
                    if len(res) < 1 or res.pop() != '(':
                        return False
                case ']':
                    if len(res) < 1 or res.pop() != '[':
                        return False
                case '}':
                    if len(res) < 1 or res.pop() != '{':
                        return False
        return True if len(res) == 0 else False

s = Solution()
print(s.isValid("()"))
print(s.isValid("()[]{}"))
print(s.isValid("([){]}"))
print(s.isValid("(]"))