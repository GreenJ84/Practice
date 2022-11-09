from collections import Counter
class Solution:
    def longestPalindrome(self, s: str) -> int:
        x, y, res = {}, list(s), 0

        for i in range(0, len(y)):
            if y[i] not in x.keys():
                x[y[i]] = 1
            else:
                x[y[i]] += 1
        for pairs in x.items():
            if pairs[1] % 2 == 0:
                res += pairs[1]
            elif pairs[1] != 1:
                res += pairs[1] // 2 * 2
                x[pairs[0]] %= 2
        if 1 in x.values():
            res += 1
        return res
    
    def longestPalindrome2(self, s: str) -> int:
        res = 0
        for x in Counter(s).values():
            res += x // 2 * 2
            if res%2==0 and x%2==1:
                res += 1
        return res

s = Solution()
print(s.longestPalindrome2("abccccdd"))
print(s.longestPalindrome2("a"))
print(s.longestPalindrome2("ccc"))
    