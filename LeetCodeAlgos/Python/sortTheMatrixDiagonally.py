# A matrix diagonal is a diagonal line of cells starting from some cell in either the topmost row or leftmost column and going in the bottom-right direction until reaching the matrix's end. For example, the matrix diagonal starting from mat[2][0], where mat is a 6 x 3 matrix, includes cells mat[2][0], mat[3][1], and mat[4][2].
# Given an m x n matrix mat of integers, sort each matrix diagonal in ascending order and return the resulting matrix.

from typing import List


class Solution:
    def diagonalSort(self, mat: List[List[int]]) -> List[List[int]]:
        m, n = len(mat), len(mat[0])
        for i in range(m - 2, -1, -1):
            for j in range(n - 2, -1, -1):
                curI, curJ = i, j
                while curI+1 < m and curJ+1 < n and mat[curI][curJ] > mat[curI + 1][curJ + 1]:
                    mat[curI][curJ], mat[curI + 1][curJ + 1] = mat[curI + 1][curJ + 1], mat[curI][curJ]
                    curI += 1
                    curJ += 1
        return mat
    
s = Solution()
# print(
#     s.diagonalSort([[1,2,3],[4,5,6],[7,8,9]]) == 
    # [[1,2,3],[4,5,6],[7,8,9]])
print(
    s.diagonalSort([[3,3,1,1], [2,2,1,2], [1,1,1,2]]) 
    == [[1,1,1,1],[1,2,2,2],[1,2,3,3]]
    )
print(
    s.diagonalSort([[11,25,66,1,69,7],[23,55,17,45,15,52],[75,31,36,44,58,8],[22,27,33,25,68,4],[84,28,14,11,5,50]]) 
    == [[5,17,4,1,52,7],[11,11,25,45,8,69],[14,23,25,44,58,15],[22,27,31,36,50,66],[84,28,75,33,55,68]]
    )
