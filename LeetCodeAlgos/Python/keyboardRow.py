# Given an array of strings words, return the words that can be typed using letters of the alphabet on only one row of American keyboard like the image below.

# In the American keyboard:

# the first row consists of the characters "qwertyuiop",
# the second row consists of the characters "asdfghjkl", and
# the third row consists of the characters "zxcvbnm".

from typing import List


class Solution:
    def findWords(self, words: List[str]) -> List[str]:
        res = []

        for word in words:
            singleRow = True
            row = ""
            if word[0].lower() in "qwertyuiop":
                row = "qwertyuiop"
            elif word[0].lower() in "asdfghjkl":
                row = "asdfghjkl"
            elif word[0].lower() in "zxcvbnm":
                row = "zxcvbnm"

            for char in word[1:].lower():
                if char not in row:
                    singleRow = False
            if singleRow:
                res.append(word)
        return res
    

s = Solution()
print(s.findWords(["Hello", "Alaska", "Dad", "Peace"]))
print(s.findWords(["onk"]))
print(s.findWords(["adsdf","sfd"]))