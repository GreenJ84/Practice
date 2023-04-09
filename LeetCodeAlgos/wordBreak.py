# Given a string s and a dictionary of strings wordDict, return true if s can be segmented into a space-separated sequence of one or more dictionary words.
# Note that the same word in the dictionary may be reused multiple times in the segmentation.

from typing import List

class Solution():
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        n = len(s)
        dp = [False] * (n+1)
        dp[0] = True
        
        for i in range(1, n+1):
            for j in range(i):
                if dp[j] and s[j:i] in wordDict:
                    dp[i] = True
                    break
        
        return dp[n]

# class Solution:
#     def wordBreak(self, s: str, wordDict: List[str]) -> bool:
#         window = ""
#         for char in s:
#             window += char
#             if window in wordDict:
#                 s = s.replace(window, "")
#                 window = ""
        
#         return True if not s else False
    
s = Solution()
print(s.wordBreak("leetcode", ["leet","code"]))
print(s.wordBreak("applepenapple",["apple","pen"]))
print(s.wordBreak("catsandog", ["cats","dog","sand","and","cat"]))