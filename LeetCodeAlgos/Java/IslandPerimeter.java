// You are given row x col grid representing a map where grid[i][j] = 1 represents land and grid[i][j] = 0 represents water.
// Grid cells are connected horizontally/vertically (not diagonally). The grid is completely surrounded by water, and there is exactly one island (i.e., one or more connected land cells).
// The island doesn't have "lakes", meaning the water inside isn't connected to the water around the island. One cell is a square with side length 1. The grid is rectangular, width and height don't exceed 100. Determine the perimeter of the island.

public class IslandPerimeter {
  public static void main(String[] args) {
    Solution s = new Solution();
    testIslandPerimeter(1, new int[][]{{0,1,0,0},{1,1,1,0},{0,1,0,0},{1,1,0,0}}, 16, s);

    testIslandPerimeter(2, new int[][]{{1}}, 4, s);

    testIslandPerimeter(3, new int[][]{{1, 0}}, 4, s);

    testIslandPerimeter(4, new int[][]{{1, 1}, {1, 1}}, 8, s);
  }

  static void testIslandPerimeter(int testNum, int[][] grid, int expected, Solution s) {
    int result = s.islandPerimeter(grid);
    System.out.println(String.format("Test %d: %d / %d (%b)", testNum, result, expected, result == expected));
    assert (result == expected);
  }
}

class Solution {
  public int islandPerimeter(int[][] grid) {
      for (int row = 0; row < grid.length; row++) {
        for (int col = 0; col < grid[0].length; col++) {
          if (grid[row][col] == 1) {
            return checkLand(grid, row, col);
          }
        }
      }
      // No land found
      return 0;
  }
  int checkLand(int[][] grid, int row, int col) {
    int landPerimeter = 4;
    grid[row][col] = -1;
    for (int[] dir : new int[][]{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}){
      int newRow = row + dir[0];
      int newCol = col + dir[1];
      if (
        // Valid row
        (newRow >= 0 && newRow < grid.length) &&
        // Valid col
        (newCol >= 0 && newCol < grid[0].length) &&
        // Land
        grid[newRow][newCol] != 0
      ){
        landPerimeter -= 1;
        if (grid[newRow][newCol] == 1){
          landPerimeter += checkLand(grid, newRow, newCol);
        }
      }
    }
    System.out.println(String.format("%d (%d, %d)", landPerimeter, row, col));
    return landPerimeter;
  }
}