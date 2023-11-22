# A fancy string is a string where no three consecutive characters are equal.

# Given a string s, delete the minimum possible number of characters from s to make it fancy.

# Return the final string after the deletion. It can be shown that the answer will always be unique.

class Solution:
    def makeFancyString(self, s: str) -> str:
        if len(s) < 3: return s
        rec = 1
        idx = 1
        while idx < len(s):
            print(idx, rec, s)
            if s[idx] == s[idx-1]:
                rec += 1
                if rec >= 3:
                    s = s[:idx] + s[idx+1:]
                    idx -= 1
            else:
                rec = 1
            idx += 1
        return s
    
s = Solution()
print(s.makeFancyString("leeetcode"))
print(s.makeFancyString("aaabaaaa"))
print(s.makeFancyString("aab"))