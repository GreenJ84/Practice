# Write a function that reverses a string. The input string is given as an array of characters s.
# You must do this by modifying the input array in-place with O(1) extra memory.

from typing import List


class Solution:
    def reverseString(self, s: List[str]) -> None:
        for i in range(len(s)//2):
            s[i], s[-(i+1)] = s[-(i+1)], s[i]
        print(s)

s = Solution()
s.reverseString(["h","e","l","l","o"])
s.reverseString(["H","a","n","n","a","h"])