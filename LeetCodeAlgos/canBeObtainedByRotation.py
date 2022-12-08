# Given two n x n binary matrices mat and target, return true if it is possible to make mat equal to target by rotating mat in 90-degree increments, or false otherwise.

from typing import List


class Solution:
    def findRotation(self, mat: List[List[int]], target: List[List[int]]) -> bool:
        if mat == target:
            return True
        for i in range(0, 3):
            mat = change(mat)
            if mat == target: 
                return True
        return False

def change(mat):
    x = zip(*mat)
    for idx, row in enumerate(x):
        mat[idx] = list(reversed(row))
    return mat

s = Solution()
print(s.findRotation([[0,1],[1,0]], [[1,0],[0,1]]))
print(s.findRotation([[0,0,0],[0,1,0],[1,1,1]],[[1,1,1],[0,1,0],[0,0,0]]))
print(s.findRotation([[0,1],[1,1]], [[1,0],[0,1]]))