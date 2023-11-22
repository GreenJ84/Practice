# You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. If a string is longer than the other, append the additional letters onto the end of the merged string.
# Return the merged string.

class Solution:
    def mergeAlternately(self, word1: str, word2: str) -> str:
        res = ''
        w1, w2 = len(word1), len(word2)
        for i in range(min(w1, w2)):
            res += word1[i]
            res += word2[i]
        if w1 < w2:
            res += word2[w1:]
        elif w1 > w2:
            res += word1[w2:]
        return res

s = Solution()

test1 = s.mergeAlternately('abc','pqr')
print(test1)
assert test1 == 'apbqcr'

test2 = s.mergeAlternately('ab','pqrs')
print(test2)
assert test2 == 'apbqrs'

test3 = s.mergeAlternately('abcd','pq')
print(test3)
assert test3 == 'apbqcd'


