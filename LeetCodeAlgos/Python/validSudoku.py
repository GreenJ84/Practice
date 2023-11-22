# Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:
    # Each row must contain the digits 1-9 without repetition.
    # Each column must contain the digits 1-9 without repetition.
    # Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
#! A Sudoku board (partially filled) could be valid but is not necessarily solvable.
#! Only the filled cells need to be validated according to the mentioned rules.


from typing import List

class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        for row in board:
            if not isValid(row):
                return False
        for col in zip(*board):
            if not isValid(col):
                return False
        for r in range(0,9,3):
            for c in range(0,9,3):
                sub = [board[r+dr][c+dc] for dr in range(3) for dc in range(3)]
                if not isValid(sub):
                    return False
        return True

def isValid(arr):
            s = ''.join(arr).replace('.','')
            return len(s) == len(set(s))

s = Solution()
print(s.isValidSudoku([["5","3",".",".","7",".",".",".","."]
,["6",".",".","1","9","5",".",".","."]
,[".","9","8",".",".",".",".","6","."]
,["8",".",".",".","6",".",".",".","3"]
,["4",".",".","8",".","3",".",".","1"]
,["7",".",".",".","2",".",".",".","6"]
,[".","6",".",".",".",".","2","8","."]
,[".",".",".","4","1","9",".",".","5"]
,[".",".",".",".","8",".",".","7","9"]]))
print(s.isValidSudoku([["8","3",".",".","7",".",".",".","."]
,["6",".",".","1","9","5",".",".","."]
,[".","9","8",".",".",".",".","6","."]
,["8",".",".",".","6",".",".",".","3"]
,["4",".",".","8",".","3",".",".","1"]
,["7",".",".",".","2",".",".",".","6"]
,[".","6",".",".",".",".","2","8","."]
,[".",".",".","4","1","9",".",".","5"]
,[".",".",".",".","8",".",".","7","9"]]))
