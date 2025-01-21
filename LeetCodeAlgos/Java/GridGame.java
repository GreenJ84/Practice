// You are given a 0-indexed 2D array grid of size 2 x n, where grid[r][c] represents the number of points at position (r, c) on the matrix. Two robots are playing a game on this matrix.

// Both robots initially start at (0, 0) and want to reach (1, n-1). Each robot may only move to the right ((r, c) to (r, c + 1)) or down ((r, c) to (r + 1, c)).

// At the start of the game, the first robot moves from (0, 0) to (1, n-1), collecting all the points from the cells on its path. For all cells (r, c) traversed on the path, grid[r][c] is set to 0. Then, the second robot moves from (0, 0) to (1, n-1), collecting the points on its path. Note that their paths may intersect with one another.

// The first robot wants to minimize the number of points collected by the second robot. In contrast, the second robot wants to maximize the number of points it collects. If both robots play optimally, return the number of points collected by the second robot.

public class GridGame {
  public static void main(String[] args) {
    GridGame obj = new GridGame();

    System.out.println(obj.gridGame(new int[][]{{3,3,1},{8,5,2}}));
    System.out.println(obj.gridGame(new int[][]{{1,3,1,15},{1,3,3,1}}));
  }

  public long gridGame(int[][] grid) {
    long firstRowSum = 0;
    for (int num : grid[0]) {
        firstRowSum += num;
    }
    long secondRowSum = 0;
    long minimumSum = Long.MAX_VALUE;
    for (int turnIndex = 0; turnIndex < grid[0].length; ++turnIndex) {
        firstRowSum -= grid[0][turnIndex];
        minimumSum = Math.min(
            minimumSum,
            Math.max(firstRowSum, secondRowSum)
        );
        secondRowSum += grid[1][turnIndex];
    }
    return minimumSum;
}
}
