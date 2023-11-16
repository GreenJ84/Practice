// A square matrix is said to be an X-Matrix if both of the following conditions hold:

// All the elements in the diagonals of the matrix are non-zero.
// All other elements are 0.
// Given a 2D integer array grid of size n x n representing a square matrix, return true if grid is an X-Matrix. Otherwise, return false.

class Solution {
    public boolean checkXMatrix(int[][] grid) {
      int n = grid.length;
      for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
          // In a Diagonal
          if (i == j || i == n - 1 - j ){
            if (grid[i][j] == 0){
              return false;
            }
          }
          // Not a diagonal index
          else {
            if (grid[i][j]!= 0){
              return false;
            }
          }
        }
      }
      return true;
    }
}

class CheckIfXMatrix {
  public static void main(String[] args){
    Solution s = new Solution();
    int[][] matrix1 = {{2,0,0,1},{0,3,1,0},{0,5,2,0},{4,0,0,2}};
    boolean result1 = s.checkXMatrix(matrix1);
    System.out.println(result1);
    assert result1 == true;

    int[][] matrix2 = {{5,7,0},{0,3,1},{0,5,0}};
    boolean result2 = s.checkXMatrix(matrix2);
    System.out.println(result2);
    assert result2 == false;
  }
}