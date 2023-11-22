# Given a string s, check if it can be constructed by taking a substring of it and appending multiple copies of the substring together.


class Solution:
    def repeatedSubstringPattern(self, s: str) -> bool:
        x = s
        for i in range(len(s)//2):
            x = x[1:]+x[:1]
            if x == s:
                return True
        return False

s = Solution()
print(s.repeatedSubstringPattern("abab"))
print(s.repeatedSubstringPattern("aba"))
print(s.repeatedSubstringPattern("abcabcabcabc"))