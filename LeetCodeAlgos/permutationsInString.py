# Given two strings s1 and s2, return true if s2 contains a permutation of s1, or false otherwise.
# In other words, return true if one of s1's permutations is the substring of s2.

from collections import Counter

class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        cntr, w = Counter(s1), len(s1)   

        for i in range(len(s2)):
            if s2[i] in cntr: 
                cntr[s2[i]] -= 1
            if i >= w and s2[i-w] in cntr: 
                cntr[s2[i-w]] += 1

            if all([cntr[i] == 0 for i in cntr]): # see optimized code below
                return True

        return False



#! Timed Out at high string lengths
# class Solution:
#     def checkInclusion(self, s1: str, s2: str) -> bool:
#         if len(s1) > len(s2):
#             return False
#         if s1 == s2 or "".join(reversed(s1)) == s2: return True

#         count = {}
#         for i in s1:
#             if i not in count:
#                 count[i] = 1
#             else:
#                 count[i]+=1

#         for i in range(len(s2)):
#             if s2[i] in s1:
#                 fallguy, j = count, i
#                 while j-i<=len(s1):
#                     if j-i == len(s1):
#                         return True
#                     if j >= len(s2) or s2[j] not in s1:
#                         break
#                     j+=1
#         return False



s = Solution()
print(s.checkInclusion("ab", "eidbaooo"))
print(s.checkInclusion("ab", "eidboaoo"))
print(s.checkInclusion("adc", "dcda"))
print(s.checkInclusion("ab", "ba"))
print(s.checkInclusion("hello", "ooolleoooleh"))
print(s.checkInclusion("abc", "bbbca"))