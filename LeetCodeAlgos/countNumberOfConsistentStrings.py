# You are given a string allowed consisting of distinct characters and an array of strings words. A string is consistent if all characters in the string appear in the string allowed.

# Return the number of consistent strings in the array words.
from typing import List 
class Solution:
    def countConsistentStrings(self, allowed: str, words: List[str]) -> int:
        res = 0
        for word in words:
            const = True
            for char in word:
                if char not in allowed:
                    const = False
                    break
            if const:
                res += 1
        return res

s = Solution()
test1 = s.countConsistentStrings(allowed = "ab", words = ["ad","bd","aaab","baa","badab"])
print(test1)
assert test1 == 2

test2 = s.countConsistentStrings(allowed = "abc", words = ["a","b","c","ab","ac","bc","abc"])
print(test2)
assert test2 == 7