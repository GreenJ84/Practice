class Solution:
    def isSubsequence(self, s, t):
        if len(s) == 0:
            return True
        elif len(s) > len(t):
            return False
        x = 0
        for i in range(0, len(t)):
            if t[i] == s[x]:
                x+=1
                if x == len(s):
                    return True
        if x != len(s):
            return False
        else:
            return True

s = Solution()
# print(s.isSubsequence('abc', 'ahbgdc'))
# print(s.isSubsequence('axc', 'ahbgdc'))
print(s.isSubsequence('', 'ahbgdc'))