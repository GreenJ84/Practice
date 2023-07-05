# There is an m x n matrix that is initialized to all 0's. There is also a 2D array indices where each indices[i] = [ri, ci] represents a 0-indexed location to perform some increment operations on the matrix.

# For each location indices[i], do both of the following:

# Increment all the cells on row ri.
# Increment all the cells on column ci.
# Given m, n, and indices, return the number of odd-valued cells in the matrix after applying the increment to all locations in indices.


from typing import List


class Solution:
    def oddCells(self, m: int, n: int, indices: List[List[int]]) -> int:
        matrix = [[0 for _ in range(n)] for _ in range(m)]
        print(matrix)

        def incrementRow(rowIdx):
            matrix[rowIdx] = list( map(lambda x: x + 1, matrix[rowIdx]) )

        def incrementCol(colIdx):
            for row in range(0, m):
                matrix[row][colIdx] = matrix[row][colIdx] + 1

        for row, col in indices:
            incrementRow(row)
            incrementCol(col)
            print(matrix)

        return sum(
            map(
                lambda row: sum(1 for _ in filter(lambda item: item % 2 == 1, row)),
                matrix
            )
        )
    
s = Solution()
print(s.oddCells(2, 3, [[0,1],[1,1]]))
print(s.oddCells(2, 2, [[1,1],[0,0]]))