# The DNA sequence is composed of a series of nucleotides abbreviated as 'A', 'C', 'G', and 'T'.
# For example, "ACGAATTCCG" is a DNA sequence.
# When studying DNA, it is useful to identify repeated sequences within the DNA.
# Given a string s that represents a DNA sequence, return all the 10-letter-long sequences (substrings) that occur more than once in a DNA molecule. You may return the answer in any order.

import functools
from typing import List

class Solution:
    def findRepeatedDnaSequences(self, s: str) -> List[str]:
        cnt = {}
        for i in range(len(s)-9):
            temp = s[i:i+10]
            print(temp)
            if temp not in cnt:
                cnt[temp] = 1
            else:
                cnt[temp] += 1
        return [k for k, v in cnt.items() if v > 1]

# class Solution:
#     def findRepeatedDnaSequences(self, s: str) -> List[str]:
#         cnt = {}
#         res = []
#         temp = "".join(s[:10])
#         cnt[temp] = 1
#         for i in range(9, len(s)-1):
#             temp = "".join(temp[1:])+s[i+1]
#             print(temp)
#             if temp not in cnt:
#                 cnt[temp] = 1
#             else:
#                 cnt[temp] += 1
#         for item in cnt.items():
#             if item[1] > 1:
#                 res.append(item[0])
#         return res
s = Solution()
print(s.findRepeatedDnaSequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"))
print(s.findRepeatedDnaSequences("AAAAAAAAAAAAA"))
print(s.findRepeatedDnaSequences("AAAAAAAAAAA"))