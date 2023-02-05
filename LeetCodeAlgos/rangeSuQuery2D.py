# Given a 2D matrix matrix, handle multiple queries of the following type:
    # Calculate the sum of the elements of matrix inside the rectangle defined by its upper left corner (row1, col1) and lower right corner (row2, col2).
# Implement the NumMatrix class:
    # NumMatrix(int[][] matrix) Initializes the object with the integer matrix matrix.
    # int sumRegion(int row1, int col1, int row2, int col2) Returns the sum of the elements of matrix inside the rectangle defined by its upper left corner (row1, col1) and lower right corner (row2, col2).
#! You must design an algorithm where sumRegion works on O(1) time complexity.

from typing import List


class NumMatrix:
    def __init__(self, matrix: List[List[int]]):
        for r in range(len(matrix)):
            for c in range(1, len(matrix[0])):
                matrix[r][c] += matrix [r][c-1]
        for r in range(1, len(matrix)):
            for c in range(len(matrix[0])):
                matrix[r][c] += matrix [r-1][c]
        self.matrix = matrix
        print(matrix)


    def sumRegion(self, row1: int, col1: int, row2: int, col2: int) -> int:
        res = self.matrix[row2][col2]
        if row1 != 0:
            res -= self.matrix[row1-1][col2]
        if col1 != 0:
            res -= self.matrix[row2][col1-1]
        if row1 != 0 and col1 != 0:
            res += self.matrix[row1-1][col1-1]
        return res


# Your NumMatrix object will be instantiated and called as such:
obj = NumMatrix(
    [
        [3, 0, 1, 4, 2],
        [5, 6, 3, 2, 1],
        [1, 2, 0, 1, 5],
        [4, 1, 0, 1, 7],
        [1, 0, 3, 0, 5]
    ]
)
print(obj.sumRegion(2, 1, 4, 3))
print(obj.sumRegion(1, 1, 2, 2))
print(obj.sumRegion(1, 2, 2, 4))