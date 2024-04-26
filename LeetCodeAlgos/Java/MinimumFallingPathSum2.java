// Given an n x n integer matrix grid, return the minimum sum of a falling path with non-zero shifts.

// A falling path with non-zero shifts is a choice of exactly one element from each row of grid such that no two elements chosen in adjacent rows are in the same column.

import java.util.*;

public class MinimumFallingPathSum2 {
  public static void main(String[] args) {
    Solution s = new Solution();

    testMinimumFallingPathSum2(1, new int[][]{{1,2,3},{4,5,6},{7,8,9}}, 13, s);

    testMinimumFallingPathSum2(2, new int[][]{{7}}, 7, s);

    testMinimumFallingPathSum2(3, new int[][]{{7, 2}, {9, 1}}, 8, s);

    testMinimumFallingPathSum2(4, new int[][]{
        {-73,61,43,-48,-36},
        {3,30,27,57,10},
        {96,-76,84,59,-15},
        {5,-49,76,31,-7},
        {97,91,61,-46,67}
      }, -192, s);

  }

  static void testMinimumFallingPathSum2(int testNum, int[][] grid, int expected, Solution s) {
    int result = s.minFallingPathSum(grid);

    System.out.println(String.format("Test %d: %d [%d, %b]", testNum, result, expected, result == expected));
    assert (result == expected);
  }
}

class Solution {
  public int minFallingPathSum(int[][] grid) {
      // First edge case, ONE row
      if (grid.length == 1) {
        return grid[0][0];
      }
      // Second edge case, TWO rows
      if (grid.length == 2) {
        return Math.min(grid[0][0] + grid[1][1], grid[0][1] + grid[1][0]);
      }
      // Get dimensions
      int n = grid.length;
      // Create a 2D array to store the minimum path sum
      // for each row bottom to top
      // without recording the last row
      int[][] dp = new int[n - 1][n];
      // Iterate all grid rows
      for (int row = n - 1; row > 0; row--) {
        // Find smallest two elements in the current grid row
        int[][] mins = new int[2][2];
        if (row == n - 1) {
          mins[0][0] = grid[row][0];
          mins[0][1] = 0;
        } else {
          mins[0][0] = dp[row][0];
          mins[0][1] = 0;
        }
        mins[1] = new int[]{ Integer.MAX_VALUE, -1 };
        for (int col = 1; col < n; col++) {
          int val = (row == n - 1 ? grid[row][col] : dp[row][col]);
          if (val < mins[0][0]) {
            mins[1][0] = mins[0][0];
            mins[1][1] = mins[0][1];
            mins[0][0] = val;
            mins[0][1] = col;
          } else if (val < mins[1][0]) {
            mins[1][0] = val;
            mins[1][1] = col;
          }
          System.out.println(Arrays.toString(mins[0]) + " " + Arrays.toString(mins[1]));
        }
        // Create a new dp array row we track (row - 1)
        dp[row - 1] = new int[n];
        // Iterate every colum space
        // Give either last row smallest or second smallest value
        for (int col = 0; col < n; col++) {
          int minLower = col == mins[0][1] ? mins[1][0] : mins[0][0];
         dp[row - 1][col] = grid[row - 1][col] + minLower;
        }
        printIntArray(dp);
      }
      // Return the minimum of the starting row
      return Arrays.stream(dp[0]).min().getAsInt();
  }

  static void printIntArray(int[][] grid) {
    for (int row = 0; row < grid.length; row++) {
      for (int col = 0; col < grid[0].length; col++) {
        System.out.print(grid[row][col] + " ");
      }
      System.out.println();
    }
    System.out.println();
  }
}