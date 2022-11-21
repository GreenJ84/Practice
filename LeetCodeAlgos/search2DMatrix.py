# Write an efficient algorithm that searches for a value target in an m x n integer matrix matrix. This matrix has the following properties:
# Integers in each row are sorted from left to right.
# The first integer of each row is greater than the last integer of the previous row.


from typing import List


class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        m, n, o = len(matrix), len(matrix[0]), 0

        for i in range(0, m):
            if i+1 == m:
                o = i
                break
            elif matrix[i+1][0] > target:
                o = i
                break
        for j in range(0, n):
            if matrix[o][j] == target:
                return True
        return False

s = Solution()
print(s.searchMatrix([[1,3,5,7],[10,11,16,20],[23,30,34,60]], 3))
print(s.searchMatrix([[1,3,5,7],[10,11,16,20],[23,30,34,60]], 13))