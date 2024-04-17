// Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:
  // Each row must contain the digits 1-9 without repetition.
  // Each column must contain the digits 1-9 without repetition.
  // Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
// Note:
  // A Sudoku board (partially filled) could be valid but is not necessarily solvable.
  // Only the filled cells need to be validated according to the mentioned rules.

import java.util.*;

public class ValidSudoku {
  
}

class Solution {
  public boolean isValidSudoku(char[][] board) {
      // Go row by row checking validity
      HashSet<Character> rowSet = new HashSet<Character>();
      List<HashSet<Character>> colSet = new ArrayList<>();
      for (int col = 0; col < 9; col++) {
        colSet.add(new HashSet<Character>());
      }
      List<HashSet<Character>> cubeSet = new ArrayList<>();
      for (int i = 0; i < 3; i++) {
        cubeSet.add(new HashSet<Character>());
      }
      for (int row = 0; row < 9; row++) {
        for (int col = 0; col < 9; col++) {
          if (board[row][col]!= '.') {
                char digit = board[row][col];
                if (rowSet.contains(digit) ||
                  colSet.get(col).contains(digit) ||
                  cubeSet.get(col / 3).contains(digit)
                ) {
                  return false;
                }
                rowSet.add(digit);
                colSet.get(col).add(digit);
                cubeSet.get(col / 3).add(digit);
              }
          }
          rowSet.clear();
          if (row % 3 == 2) {
            for (int i = 0; i < 3; i++) {
              cubeSet.get(i).clear();
            }
          }
      }
      return true;
  }
}