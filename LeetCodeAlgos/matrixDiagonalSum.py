# Given a square matrix mat, return the sum of the matrix diagonals.
# Only include the sum of all the elements on the primary diagonal and all the elements on the secondary diagonal that are not part of the primary diagonal.

from typing import List


class Solution:
    def diagonalSum(self, mat: List[List[int]]) -> int:
        x, y, sum = 0, len(mat[0])-1, 0
        for i in mat:
            sum += i[x]+i[y] if x != y else i[x]
            x += 1
            y -= 1
        return sum

s = Solution()
print(s.diagonalSum([[1,2,3],[4,5,6],[7,8,9]]))
print(s.diagonalSum([[1,1,1,1],[1,1,1,1],[1,1,1,1],[1,1,1,1]]))
print(s.diagonalSum([[5]]))