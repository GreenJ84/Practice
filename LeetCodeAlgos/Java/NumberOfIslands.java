// Given an m x n 2D binary grid grid which represents a map of '1's (land) and '0's (water), return the number of islands.

// An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.

public class NumberOfIslands {
  public static void main(String[] args) {
    Solution s = new Solution();

    testNumIslands(1, new char[][]{{'1','1','1','1','0'},{'1','1','0','1','0'},{'1','1','0','0','0'},{'0','0','0','0','0'}}, 1, s);
    testNumIslands(2, new char[][]{{'1','1','0','0','0'},{'1','1','0','0','0'},{'0','0','1','0','0'},{'0','0','0','1','1'}}, 3, s);
    testNumIslands(3, new char[][]{{'1'}}, 1, s);

    testNumIslands(4, new char[][]{{'0'}}, 0, s);
  }

  static void testNumIslands(int testNum, char[][] grid, int expected, Solution solution) {
    int result = solution.numIslands(grid);

    System.out.println(String.format(
      "Test %d: %d / %d (%b)",
      testNum,
      result,
      expected,
      result == expected
    ));
  }

}

class Solution {
  public int numIslands(char[][] grid) {
    int count = 0;
      for (int row = 0; row < grid.length; row++) {
        for (int col = 0; col < grid[0].length; col++) {
          if (grid[row][col] == '1') {
            count++;
            cancelIsland(grid, row, col);
          }
        }
      }
      return count;
  }
  public void cancelIsland(char[][] grid, int row, int col) {
    int[][] directions = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};
    for (int[] direction : directions) {
      int newRow = row + direction[0];
      int newCol = col + direction[1];
      if (newRow >= 0 && newRow < grid.length && newCol >= 0 && newCol < grid[0].length && grid[newRow][newCol] == '1') {
        grid[newRow][newCol] = '0';
        cancelIsland(grid, newRow, newCol);
      }
    }
  }
}
