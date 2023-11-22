# You are given two strings s1 and s2 of equal length. A string swap is an operation where you choose two indices in a string (not necessarily different) and swap the characters at these indices.
# Return true if it is possible to make both strings equal by performing at most one string swap on exactly one of the strings. Otherwise, return false.

class Solution:
    def areAlmostEqual(self, s1: str, s2: str) -> bool:
        if s1 == s2:
            return True
        x, y, z = 0, 0, 0
        for i in range(0, len(s1)):
            if s1[i] != s2[i] and z == 0:
                x, y = s1[i], s2[i]
                z+=1
            elif s1[i] != s2[i] and z == 1:
                if s1[i] == y and s2[i] == x:
                    z+=1
                else: 
                    return False
            elif s1[i] != s2[i] and z > 1:
                return False
        return True if z == 2 else False

s = Solution()
print(s.areAlmostEqual('kanb', 'bank'))
print(s.areAlmostEqual('attack', 'defend'))
print(s.areAlmostEqual('kelb', 'kelb'))
