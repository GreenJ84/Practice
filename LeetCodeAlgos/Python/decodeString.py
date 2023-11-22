# Given an encoded string, return its decoded string.
# The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is being repeated exactly k times. Note that k is guaranteed to be a positive integer.
# You may assume that the input string is always valid; there are no extra white spaces, square brackets are well-formed, etc. Furthermore, you may assume that the original data does not contain any digits and that digits are only for those repeat numbers, k. For example, there will not be input like 3a or 2[4].
# The test cases are generated so that the length of the output will never exceed 105.

class Solution:
    def decodeString(self, s: str) -> str:
        res = ''
        stack = []

        for c in s:
            if c.isdigit():
                if len(res)!=0 and not res.isdigit():
                    stack.append(res)
                    res=''
                res+=c
            elif c == '[':
                stack.append(res)
                res=''
            elif c == ']':
                if not stack[-1].isdigit():
                    res = stack.pop()+res
                stack.append(res)
                res=''
                str = stack.pop()
                rep = stack.pop()
                for i in range(int(rep)):
                    res+=str
            else:
                res+=c
        stack.append(res)
        return ''.join(stack)



s = Solution()
print(s.decodeString("3[a]2[bc]"))
print(s.decodeString("3[a2[c]]"))
print(s.decodeString("2[abc]3[cd]ef"))
