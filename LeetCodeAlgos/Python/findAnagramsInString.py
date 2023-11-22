# Given two strings s and p, return an array of all the start indices of p's anagrams in s. You may return the answer in any order.
# An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
from typing import List
from collections import Counter


class Solution:
    def findAnagrams(self, s: str, p: str) -> List[int]:
        n, m = len(s), len(p)
        # Hold the count for substring
        ref = Counter(p)
        # Hold the count for the current window
        curr = Counter(s[:m])
        ans=[]
        if curr == ref:
            ans.append(0)

        # iterate the window
        for i in range(m,n):
            curr[s[i]] += 1
            curr[s[i-m]] -= 1
            if curr == ref:
                ans.append(i-m+1)
        return ans

s = Solution()
print(s.findAnagrams("cbaebabacd", "abc"))

