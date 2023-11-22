# You are given an array of n strings strs, all of the same length.
# The strings can be arranged such that there is one on each line, making a grid. 
# You want to delete the columns that are not sorted lexicographically. In the above example (0-indexed), columns 0 ('a', 'b', 'c') and 2 ('c', 'e', 'e') are sorted while column 1 ('b', 'c', 'a') is not, so you would delete column 1.
# Return the number of columns that you will delete.

from typing import List


class Solution:
    def minDeletionSize(self, strs: List[str]) -> int:
        alpha=["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"]
        bad = 0
        for x in range(len(strs[0])):
            for y in range(1, len(strs)):
                s, n = alpha.index(strs[y][x]), alpha.index(strs[y-1][x])
                if not alpha.index(strs[y][x])>alpha.index(strs[y-1][x]):
                    bad+=1
                    break
        return bad

s = Solution()
print(s.minDeletionSize(["cba","daf","ghi"]))
print(s.minDeletionSize(["a","b"]))
print(s.minDeletionSize(["zyx","wvu","tsr"]))


