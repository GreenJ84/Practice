# Given an m x n matrix board containing 'X' and 'O', capture all regions that are 4-directionally surrounded by 'X'.
# A region is captured by flipping all 'O's into 'X's in that surrounded region.

from typing import List


class Solution:
    def solve(self, board: List[List[str]]) -> None:
        rows, cols = len(board), len(board[0])
        seen = set()
        offSet = [(-1, 0), (0, 1), (1, 0), (0, -1)]

        def color(_row, _col):
            seen.add((_row, _col))
            for xOff, yOff in offSet:
                nRow, nCol = _row + xOff, _col + yOff
                if (0 <= nRow <= rows-1 and 0 <= nCol <= cols-1 and board[nRow][nCol] == "O" and (nRow, nCol) not in seen):
                    color(nRow, nCol)
            return


        for col in range(cols):
            if board[0][col] == "O" and (0, col) not in seen:
                color(0, col)
            if board[rows-1][col] == "O" and (rows-1, col) not in seen:
                color(rows-1, col)

        for row in range(rows):
            if board[row][0] == "O" and (row, 0) not in seen:
                color(row, 0)
            if board[row][cols-1] == "O" and (row, cols-1) not in seen:
                color(row, cols-1)

        for row in range(1, rows-1):
            for col in range(1, cols-1):
                if board[row][col] == "O" and (row, col) not in seen:
                    board[row][col] = "X"

s = Solution()
# test = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
# s.solve(test)
# print(test)

# test = [["X"]]
# s.solve(test)
# print(test)

test = [["O","O","O"],["O","O","O"],["O","O","O"]]
s.solve(test)
print(test)