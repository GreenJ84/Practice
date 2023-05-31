# Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.
# You must do it in place.

from typing import List


class Solution:
    def setZeroes(self, matrix: List[List[int]]) -> None:
        zeros = []
        i = 0
        while i < len(matrix):
            for j in range(len(matrix[0])):
                if matrix[i][j] == 0:
                    zeros.append((i, j))
            i += 1

        rows = []
        cols = []
        for zero in zeros:
            if zero[0] not in rows:
                rows.append(zero[0])
                matrix[zero[0]] = [0] * len(matrix[0])
            if zero[1] not in cols:
                cols.append(zero[1])
                for i in range(len(matrix)):
                    matrix[i][zero[1]] = 0

s = Solution()

matrix = [[1,1,1],[1,0,1],[1,1,1]]
s.setZeroes(matrix)
print(matrix)
assert matrix == [[1,0,1],[0,0,0],[1,0,1]]

matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
s.setZeroes(matrix)
print(matrix)
assert matrix == [[0,0,0,0],[0,4,5,0],[0,3,1,0]]