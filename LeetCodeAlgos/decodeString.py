# Given an encoded string, return its decoded string.
# The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is being repeated exactly k times. Note that k is guaranteed to be a positive integer.
# You may assume that the input string is always valid; there are no extra white spaces, square brackets are well-formed, etc. Furthermore, you may assume that the original data does not contain any digits and that digits are only for those repeat numbers, k. For example, there will not be input like 3a or 2[4].
# The test cases are generated so that the length of the output will never exceed 105.

class Solution:
    def decodeString(self, s: str) -> str:
        res = ''
        for i in range(0, len(s)):
            if s[i] == '[':
                temp, j = '', i
                while(j != ']'):
                    temp += s[j]
                res += breakdown(i-1, temp)

def breakdown(rep, val):
    for i in range(0, len(val)):
        res = ''
        if s[i] == '[':
            temp, j = '', i
            while(j != ']'):
                temp += s[j]
            res += breakdown(i-1, temp)
        

s = Solution()
print(s.decodeString("3[a]2[bc]"))
print(s.decodeString("3[a2[c]]"))
print(s.decodeString("2[abc]3[cd]ef"))
