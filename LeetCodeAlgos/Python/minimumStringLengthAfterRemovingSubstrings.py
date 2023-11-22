# You are given a string s consisting only of uppercase English letters.
# You can apply some operations to this string where, in one operation, you can remove any occurrence of one of the substrings "AB" or "CD" from s.
# Return the minimum possible length of the resulting string that you can obtain.
# Note that the string concatenates after removing the substring and could produce new "AB" or "CD" substrings.

class Solution:
    def minLength(self, s: str) -> int:
        s = list(s)
        idx = 0
        while idx < len(s) - 1:
            if (s[idx] == "A" and s[idx+1] == "B") or (s[idx] == "C" and s[idx+1] == "D"):
                s.pop(idx)
                s.pop(idx)
                idx = max(idx - 1, 0)
                print(s)
            else:
                idx += 1
        return len(s)

s = Solution()
print(s.minLength("ABFCACDB"))
print(s.minLength("ACBBD"))
print(s.minLength("CCCACCACCCAAACDBBBDDDBDDBDDDBIAABA"))