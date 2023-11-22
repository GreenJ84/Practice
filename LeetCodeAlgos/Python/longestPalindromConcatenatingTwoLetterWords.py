# You are given an array of strings words. Each element of words consists of two lowercase English letters.
# Create the longest possible palindrome by selecting some elements from words and concatenating them in any order. Each element can be selected at most once.
# Return the length of the longest palindrome that you can create. If it is impossible to create any palindrome, return 0.
# A palindrome is a string that reads the same forward and backward.

from typing import List


class Solution:
    def longestPalindrome(self, words: List[str]) -> int:
        hold = {}
        res = 0
        for w in words:
            y = ""
            if w[0] != w[1]:
                y = "".join(list(reversed(w)))
            if w in hold:
                hold[w][1] += 1
            elif y in hold:
                hold[y][0] += 1
            elif w not in hold:
                hold[w] = [0, 1]
        words = [list(item) for item in sorted(hold.items(), key = lambda x: x[1])]

        mid = False
        for item in words:
            if item[0][0] == item[0][1]:
                if not mid and item[1][1] % 2 == 1:
                    res += 2
                    mid = True
                res += item[1][1]//2*4
            else:
                res += min(item[1][0], item[1][1])*4
        return res


s = Solution()
# print(s.longestPalindrome(["lc","cl","gg"]))
# print(s.longestPalindrome(["ab","ty","yt","lc","cl","ab"]))
# print(s.longestPalindrome(["cc","ll","xx"]))
# print(s.longestPalindrome(["cl", "cl", "cl", "lc", "lc", "lc", "gg"]))
# print(s.longestPalindrome(["dd","aa","bb","dd","aa","dd","bb","dd","aa","cc","bb","cc","dd","cc"]))
print(s.longestPalindrome(["nt","tt","yt","yn","yy","yn","nt","yn","nt","yy","ny","yn","ny","nn","nn","yy","nn","yt","yn","tn","yy","yn","nt","tt","yn","ny","nn","yn","nn","nt","ny","ty","ny","nt","ny","tn","nt","yy","nn","yy","nn","yt","yy","yn","nn","ny","ty","nn","nt","nt","ny","yy","tn","yy","tn","ty","ny","nt","ty","yt","yn","nn","ny","yt","nt","tt","nt","yy","yt","nt","tt","yn","tn","tn","nt","yt","yn","ty","yy","ty","tn","tn","yn","ty","ty","tn","nn","nt","nn","nn","tt","yy","ty","ny","yy","ny"]))