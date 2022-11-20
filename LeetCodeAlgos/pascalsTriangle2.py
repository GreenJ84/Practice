# Given an integer rowIndex, return the rowIndexth (0-indexed) row of the Pascal's triangle.
# In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:

from typing import List


class Solution:
    def getRow(self, rowIndex: int) -> List[int]:
        res = [1]
        for i in range(0, rowIndex):
            res = newRow(res)
        return res
def newRow(old):
    new = [1]
    for i in range(0, len(old)-1):
        new.append(old[i]+old[i+1])
    new.append(1)
    return new
    
s = Solution()
print(s.getRow(0))
print(s.getRow(2))
print(s.getRow(3))
print(s.getRow(5))