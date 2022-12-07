# Given a string s, find the length of the longest substring without repeating characters.

class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        res, word, i = 0, [], 0
        while i < len(s):
            if s[i] in word:
                if len(word)>res:
                    res=len(word)
                i = i-len(word)+1
                word.clear()
            word.append(s[i])
            i+=1
        return max(res, len(word))

s = Solution()
print(s.lengthOfLongestSubstring("abcabcbb"))
print(s.lengthOfLongestSubstring("bbbbb"))
print(s.lengthOfLongestSubstring("pwwkew"))
print(s.lengthOfLongestSubstring(" "))
print(s.lengthOfLongestSubstring("dvdf"))

