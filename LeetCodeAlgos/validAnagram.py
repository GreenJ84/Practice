# Given two strings s and t, return true if t is an anagram of s, and false otherwise.
# An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        if len(t) != len(s):
            return False
        ori, sub = {}, {}
        for i in s:
            if i not in ori:
                ori[i] = 1
            else:
                ori[i] +=1
        for j in t:
            if j not in sub:
                sub[j] = 1
            else:
                sub[j] +=1
        for y in t:
            if y not in ori or ori[y] < sub[y]:
                return False
        return True

s = Solution()
print(s.isAnagram('anagram', 'nagaram'))
print(s.isAnagram('rat', 'car'))
print(s.isAnagram('ab', 'a'))