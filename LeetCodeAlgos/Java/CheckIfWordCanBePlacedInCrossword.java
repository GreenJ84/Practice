// You are given an m x n matrix board, representing the current state of a crossword puzzle. The crossword contains lowercase English letters (from solved words), ' ' to represent any empty cells, and '#' to represent any blocked cells.

// A word can be placed horizontally (left to right or right to left) or vertically (top to bottom or bottom to top) in the board if:

// It does not occupy a cell containing the character '#'.
// The cell each letter is placed in must either be ' ' (empty) or match the letter already on the board.
// There must not be any empty cells ' ' or other lowercase letters directly left or right of the word if the word was placed horizontally.
// There must not be any empty cells ' ' or other lowercase letters directly above or below the word if the word was placed vertically.
// Given a string word, return true if word can be placed in board, or false otherwise.

public class CheckIfWordCanBePlacedInCrossword {
  public static void main(String[] args) {
    CheckIfWordCanBePlacedInCrossword obj = new CheckIfWordCanBePlacedInCrossword();

    char[][] board = {{'#', ' ', '#'}, {' ', ' ', '#'}, {'#', 'c', ' '}};
    obj.testIfWordCanBePlacedInCrossword(1, board, "abc", true);

    char[][] board2 = {{' ', '#', 'a'}, {' ', '#', 'c'}, {' ', '#', 'a'}};
    obj.testIfWordCanBePlacedInCrossword(2, board2, "ac", false);


    char[][] board3 = {{'#', ' ', '#'}, {' ', ' ', '#'}, {'#', ' ', 'c'}};
    obj.testIfWordCanBePlacedInCrossword(3, board3, "ca", true);
  }

  private void testIfWordCanBePlacedInCrossword(
    int testNum,
    char[][] board,
    String word,
    boolean expected
  ){
    boolean result = this.placeWordInCrossword(board, word);
    System.out.printf("Test %d: %b (%s)\n", testNum, result, expected == result ? "PASS" : "FAIL");
  }

  private int rows;
  private int cols;
  private char[][] board;
  private String word;
  public boolean placeWordInCrossword(char[][] board, String word) {
      this.rows = board.length;
      this.cols = board[0].length;
      this.board = board;
      this.word = word;

      for (int row = 0; row < rows; row++) {
          for (int col = 0; col < cols; col++) {
            char ch = board[row][col];
            if (ch != '#' && (ch == ' ' || ch == word.charAt(0))) {
                if (isPossible(row, col)) {
                    return true;
                }
            }
          }
      }
      return false;
  }

  // Dfs all 4 directions to see if possible to fit word
  private boolean isPossible(int row, int col){
    for (int[] dir : new int[][]{{0,1}, {1,0}, {0, -1}, {-1, 0}}){
      if (dfs(row, col, dir[0], dir[1])){
        return true;
      }
    }
    return false;
  }

  // Fully search a single direction off of a single starting entry
  private boolean dfs(int row, int col, int dx, int dy) {
    int prevRow = row + dy * -1;
    int prevCol = col + dx * -1;
    if (!isEnd(prevRow, prevCol)) {
      return false;
    }
    int len = this.word.length();
    for (int idx = 1; idx < len; idx++) {
      int newRow = row + dy * idx;
      int newCol = col + dx * idx;
      if (isEnd(newRow, newCol) || this.board[newRow][newCol] != ' ' && this.board[newRow][newCol] != word.charAt(idx)) {
        return false;
      }
    }
    int afterRow = row + dx * len;
    int afterCol = col + dy * len;
    return isEnd(afterRow, afterCol);
  }

  // Check if the new position is within the board boundaries
  private boolean isWithinBoard(int row, int col){
    return (row >= 0 && row < this.rows) && (col >= 0 && col < this.cols);
  }

  // Check if space is a valid ending space
  private boolean isEnd(int row, int col){
    return !isWithinBoard(row, col) || this.board[row][col] == '#';
  }
}
