# In an alien language, surprisingly, they also use English lowercase letters, but possibly in a different order. The order of the alphabet is some permutation of lowercase letters.
# Given a sequence of words written in the alien language, and the order of the alphabet, return true if and only if the given words are sorted lexicographically in this alien language.

from typing import List


class Solution:
    def isAlienSorted(self, words: List[str], order: str) -> bool:
        for i in range(len(words)-1):

            if order.index(words[i][0]) > order.index(words[i+1][0]):
                return False

            elif order.index(words[i][0]) == order.index(words[i+1][0]):
                s = 1
                while s < min(len(words[i]), len(words[i+1])):
                    if order.index(words[i][s]) > order.index(words[i+1][s]):
                        return False
                    elif order.index(words[i][s]) < order.index(words[i+1][s]):
                        break
                    s+=1
                if s == len(words[i+1]) and s < len(words[i]):
                    return False
        return True

s = Solution()
# print(s.isAlienSorted(["hello","leetcode"],"hlabcdefgijkmnopqrstuvwxyz"))
# print(s.isAlienSorted(["word","world","row"], "worldabcefghijkmnpqstuvxyz"))
# print(s.isAlienSorted(["apple","app"], "abcdefghijklmnopqrstuvwxyz"))
print(s.isAlienSorted(["apap","app"], "abcdefghijklmnopqrstuvwxyz"))
