# Given a pattern and a string s, find if s follows the same pattern.
# Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in s.

class Solution:
    def wordPattern(self, pattern: str, s: str) -> bool:
        pattern = list(pattern)
        s = s.split(" ")
        if len(pattern) != len(s):
            return False

        con = {}
        con[pattern[0]] = s[0]
        for i in range(1, len(s)):
            if pattern[i] not in con:
                if s[i] in con.values():
                    return False
                con[pattern[i]] = s[i]
            else:
                if con[pattern[i]] != s[i]:
                    return False
        return True

s = Solution()
print(s.wordPattern('abba', 'dog cat cat dog'))
print(s.wordPattern('abba', 'dog dog dog dog'))
print(s.wordPattern('abba', 'dog cat cat fish'))
print(s.wordPattern('aaaa', 'dog cat cat dog'))