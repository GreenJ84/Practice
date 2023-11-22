# Given an integer numRows, return the first numRows of Pascal's triangle.
# In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:

from typing import List


class Solution:
    def generate(self, numRows: int) -> List[List[int]]:
        res = [[1]]
        for i in range(1, numRows):
            res.append(newRow(res[i-1]))
        return res
def newRow(old):
    new = [1]
    for i in range(0, len(old)-1):
        new.append(old[i]+old[i+1])
    new.append(1)
    return new
    
s = Solution()
print(s.generate(3))
print(s.generate(4))
print(s.generate(5))
print(s.generate(6))
