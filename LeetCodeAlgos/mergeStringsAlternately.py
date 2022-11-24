# You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. If a string is longer than the other, append the additional letters onto the end of the merged string.
# Return the merged string.

class Solution:
    def mergeAlternately(self, word1: str, word2: str) -> str:
        res = ''
        for i in range(0, min(len(word1), len(word2))):
            res+=word1[i]
            res+=word2[i]
        if len(word1)>len(word2):
            res+=word1[slice(i+1, len(word1))]
        elif len(word1)<len(word2):
            res+=word2[slice(i+1, len(word2))]
        return res

s = Solution()
print(s.mergeAlternately('abc','pqr'))
print(s.mergeAlternately('ab','pqrs'))
print(s.mergeAlternately('abcd','pq'))


