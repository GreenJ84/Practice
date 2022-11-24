# You are given an array of characters 'letters' that is sorted in non-decreasing order, and a character target. There are at least two different characters in letters.
# Return the smallest character in letters that is lexicographically greater than target. If such a character does not exist, return the first character in letters.

from math import inf
from typing import List


class Solution:
    def nextGreatestLetter(self, letters: List[str], target: str) -> str:
        abc =['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z']

        place = abc.index(target)+1
        while place<26 and abc[place] not in letters:
            place+=1
        return letters[letters.index(abc[place])] if place<26 else letters[0]

s = Solution()
# print(s.nextGreatestLetter(["c","f","j"], 'c'))
print(s.nextGreatestLetter(["x","x","y","y"], 'z'))
# print(s.nextGreatestLetter(["p","r","s","t","u","v","w","x","y","z"], 'p'))
