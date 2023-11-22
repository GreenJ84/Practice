# You are given a string s formed by digits and '#'. We want to map s to English lowercase characters as follows:
    # Characters ('a' to 'i') are represented by ('1' to '9') respectively.
    # Characters ('j' to 'z') are represented by ('10#' to '26#') respectively.
# Return the string formed after mapping.
# The test cases are generated so that a unique mapping will always exist.

class Solution:
    def freqAlphabets(self, s: str) -> str:
        alphabet = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z']
        res = ''
        i = len(s)-1
        while i>=0:
            if s[i] =='#':
                res = alphabet[int(s[i-2]+s[i-1])-1] + res
                i-=2
            else:
                res = alphabet[int(s[i])-1] + res
            i-=1
        return res

s = Solution()
print(s.freqAlphabets("10#11#12"))
print(s.freqAlphabets("1326#"))