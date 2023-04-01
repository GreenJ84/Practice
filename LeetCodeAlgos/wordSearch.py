# Given an m x n grid of characters board and a string word, return true if word exists in the grid.
# The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once.

from typing import List


class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]
        end = len(word)
        rows, cols = len(board), len(board[0])
        def checkWord(idx, row, col, seen):
            exists = False
            seen += [(row, col)]
            if idx == end:
                return True
            for xB, yB in directions:
                rowN = row+xB
                colN = col+yB
                if 0 <= rowN < rows and 0 <= colN < cols and board[rowN][colN] == word[idx] and (rowN, colN) not in seen:
                    if checkWord(idx+1, row+xB, col+yB, seen[:]):
                        exists = True
            return exists

        for row in range(rows):
            for col in range(cols):
                if board[row][col] == word[0]:
                    if checkWord(1, row, col, []):
                        return True
        return False

s = Solution()
print(s.exist([["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], "ABCCED"))
print(s.exist([["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], "SEE"))
print(s.exist([["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], "ABCB"))