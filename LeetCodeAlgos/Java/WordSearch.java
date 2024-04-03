// Given an m x n grid of characters board and a string word, return true if word exists in the grid.

// The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once.

class Solution {
  static final int[][] DIRECTIONS = { {-1, 0}, {0, 1}, {0, -1}, {1, 0} };
  public boolean exist(char[][] board, String word) {
    int m = board.length;
    int n = board[0].length;
    for (int row = 0; row < m; row++) {
      for (int col = 0; col < n; col++) {
        if (board[row][col] == word.charAt(0) && search(board, word, row, col, 1, new boolean[m][n])) {
          return true;
        }
      }
    }
    return false;
  }

  public boolean search(char[][] board, String word, int row, int col, int index, boolean[][] visited) {
    if (index == word.length()) {return true;}
    visited[row][col] = true;
    for (int[] direction: DIRECTIONS) {
      int newRow = row + direction[0];
      int newCol = col + direction[1];
      // Index Error handing
      if (newRow < 0 || newRow >= board.length || newCol < 0 || newCol >= board[0].length) {
        continue;
      }
      // Word searching
      else if (board[newRow][newCol] == word.charAt(index) &&!visited[newRow][newCol]){
        if (search(board, word, newRow, newCol, index + 1, deepCopy(visited))){
          return true;
        }
      }
      if (visited[newRow][newCol]) {
      }
      if (board[newRow][newCol] != word.charAt(index)){
      }
    }

    return false;
  }

  private boolean[][] deepCopy(boolean[][] original) {
    boolean[][] copy = new boolean[original.length][];
    for (int i = 0; i < original.length; i++) {
        copy[i] = original[i].clone();
    }
    return copy;
  }
}

class WordSearch {
  public static void main(String[] args) {
    Solution solution = new Solution();
    char[][] board = {{'A', 'B', 'C', 'E'}, {'S', 'F', 'C', 'S'}, {'A', 'D', 'E', 'E'}};
    System.out.println(solution.exist(board, "ABCCED"));

    System.out.println(solution.exist(board, "SEE"));

    System.out.println(solution.exist(board, "ABCB"));

    char[][] board2 = {{'A', 'B', 'C', 'E'}, {'S', 'F', 'E', 'S'}, {'A', 'D', 'E', 'E'}};
    System.out.println(solution.exist(board2, "ABCESEEEFS"));
  }
}