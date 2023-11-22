# In MATLAB, there is a handy function called reshape which can reshape an m x n matrix into a new one with a different size r x c keeping its original data.
# You are given an m x n matrix mat and two integers r and c representing the number of rows and the number of columns of the wanted reshaped matrix.
# The reshaped matrix should be filled with all the elements of the original matrix in the same row-traversing order as they were.
# If the reshape operation with given parameters is possible and legal, output the new reshaped matrix; Otherwise, output the original matrix.

from typing import List


class Solution:
    def matrixReshape(self, mat: List[List[int]], r: int, c: int) -> List[List[int]]:
        if len(mat)*len(mat[0]) != r*c:
            return mat
        res, lvl, s = [], [], 0
        for i in mat:
            for j in range(0, len(mat[0])):
                if s != 0 and s%c == 0:
                    res.append(lvl)
                    lvl = []
                lvl.append(i[j])
                s+=1
        res.append(lvl)
        return res

s = Solution()
# print(s.matrixReshape([[1,2],[3,4]], 1, 4))
# print(s.matrixReshape([[1,2],[3,4]], 2, 4))
print(s.matrixReshape([[1,2,3],[4,5,6]], 3, 2))