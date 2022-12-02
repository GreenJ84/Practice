# Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

class Solution:
    def strStr(self, haystack: str, needle: str) -> int:
        i = 0
        while i < len(haystack):
            if haystack[i] == needle[0]:
                s, j, pos = 1, i+1, i
                while j < len(haystack) and s < len(needle) and haystack[j]==needle[s]:
                    s+=1
                    j+=1
                if s == len(needle):
                    return pos
            i+=1
        return -1

s = Solution()
print(s.strStr("sadbutsad", "sad"))
print(s.strStr("leetcode", "leto"))
