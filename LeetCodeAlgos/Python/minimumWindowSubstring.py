# Given two strings s and t of lengths m and n respectively, return the minimum window substring of s such that every character in t (including duplicates) is included in the window. If there is no such substring, return the empty string "".
# The testcases will be generated such that the answer is unique.

class Solution:
    def minWindow(self, s: str, t: str) -> str:
        # If t is longer no possible solution
        if len(t) > len(s): return ""

        if len(t) == len(s):
        # each character should be the same when sorted
            listS = sorted(list(s))
            t = sorted(list(t))
            for i in range(len(t)):
                if t[i] != listS[i]: 
                    return ""
            return s

        res = ""
        # Window starting point through s
        for i in range(len(s)):
            # Continue if not valid
            if s[i] not in t:
                continue
            # A target list to hold target characters
            target = list(t)
            # variable to hold the str window
            temp = ""
            # expand the window until full
            # carry the i variable to reset window start on full window
            while i < len(s):
                if s[i] in target:
                    target.remove(s[i])
                temp += s[i]
                if not target:
                    if not res or len(temp) < len(res):
                        res = temp
                    break
                i+=1
        return res

s = Solution()
print(s.minWindow("ADOBECODEBANC", "ABC"))
print(s.minWindow("a", "a"))
print(s.minWindow("a", "aa"))
print(s.minWindow("ab", "a"))
print(s.minWindow("ab", "b"))
print(s.minWindow("ab", "A"))
print(s.minWindow("abc", "cba"))
