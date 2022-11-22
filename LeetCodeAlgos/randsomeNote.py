# Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.
# Each letter in magazine can only be used once in ransomNote.

class Solution:
    def canConstruct(self, ransomNote: str, magazine: str) -> bool:
        if len(ransomNote) > len(magazine):
            return False
        note, mag = {}, {}
        for i in ransomNote:
            if i not in note:
                note[i] = 1
            else:
                note[i] +=1
        for j in magazine:
            if j not in mag:
                mag[j] = 1
            else:
                mag[j] +=1
        for y in ransomNote:
            if y not in mag or note[y] > mag[y]:
                return False
        return True

s = Solution()
print(s.canConstruct('a', 'b'))
print(s.canConstruct('aa', 'ab'))
print(s.canConstruct('aa', 'aab'))

