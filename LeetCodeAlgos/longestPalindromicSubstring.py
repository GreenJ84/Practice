# Given a string s, return the longest palindromic substrin in s.

class Solution:
    def longestPalindrome(self, s: str) -> str:
        if len(s) < 2: return s
        start, end = 0, 0
        for i in range(len(s)):
            len1 = self.__expandAroundCenter(s, i, i)
            len2 = self.__expandAroundCenter(s, i, i+1)
            lenT = max(len1, len2)
            if lenT > end - start:
                start = int(i - (lenT - 2) / 2)
                end = int(i + lenT / 2)
        return s[start:end+1]


    def __expandAroundCenter(self, s, left, right):
        l = left
        r = right
        while l >= 0 and r < len(s) and s[l] == s[r]:
            r+=1
            l-=1
        return r-l-1

s = Solution()
print(s.longestPalindrome("babad"))
print(s.longestPalindrome("cbbd"))