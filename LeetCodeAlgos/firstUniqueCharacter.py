# Given a string s, find the first non-repeating character in it and return its index. If it does not exist, return -1.

class Solution:
    def firstUniqChar(self, s: str) -> int:
        s = list(s)
        rep, sin = [], []
        for i in range(0, len(s)):
            if not s[i] in sin and not s[i] in rep:
                sin.append(s[i])
            elif s[i] in sin and not s[i] in rep:
                sin.remove(s[i])
                rep.append(s[i])

        return s.index(sin[0]) if len(sin) > 0 else -1

s = Solution()
print(s.firstUniqChar('leetcode'))
print(s.firstUniqChar('loveleetcode'))
print(s.firstUniqChar('aabb'))