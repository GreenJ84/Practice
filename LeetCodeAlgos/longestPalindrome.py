

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

s = Solution()
# print(s.longestPalindrome("abccccdd"))
# print(s.longestPalindrome("a"))
print(s.longestPalindrome("ccc"))
    