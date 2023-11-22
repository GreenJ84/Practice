class Solution:
    def isIsomorphic(self, s, t):
        if len(s) != len(t):
            return False
        dict = {}
        for i in range(0, len(s)):
            if s[i] not in dict:
                if t[i] not in dict.values():
                    dict[s[i]] = t[i]
                else:
                    return False
            elif t[i] != dict[s[i]]:
                return False
        return True

s = Solution()
print(s.isIsomorphic('egg', 'add'))
print(s.isIsomorphic('foo', 'bar'))
print(s.isIsomorphic('paper', 'title'))
print(s.isIsomorphic('badc', 'baba'))