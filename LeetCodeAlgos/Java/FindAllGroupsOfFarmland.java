import java.util.*;

public class FindAllGroupsOfFarmland{
  public static void main(String[] args) {
    Solution s = new Solution();

    testFindFarmland(1, new int[][]{{1, 0, 0}, {0, 1, 1}, {0, 1, 1}}, s);

    testFindFarmland(2, new int[][]{{1, 1}, {1, 1},}, s);
    
    testFindFarmland(3, new int[][]{{0}}, s);
  }

  static void testFindFarmland(int testNum, int[][] land, Solution s) {
    int[][] result = s.findFarmland(land);
    System.out.println(String.format("Test %d:", testNum));
    for (int[] r : result){
      System.out.println(Arrays.toString(r));
    }
    System.out.println();

  }
}

class Solution {
  public int[][] findFarmland(int[][] land) {
      ArrayList<int[]> result = new ArrayList<>();
      for (int row = 0; row < land.length; row++) {
        for (int col = 0; col < land[0].length; col++) {
          if (land[row][col] == 1) {
            result.add(bottomRightCorner(land, new int[]{row, col, 0, 0}, row, col));
          }
        }
      }

      return result.toArray(new int[result.size()][4]);
  }

  static int[] bottomRightCorner(int[][] land, int[] farmCoords, int row, int col){
    land[row][col] = 0;
    // System.out.println(String.format("Seeing - [%d, %d]", row, col));
    if (row + 1 < land.length && land[row + 1][col] == 1) {
      // System.out.println(String.format("Down - [%d, %d]", row, col));
      return bottomRightCorner(land, farmCoords, row + 1, col);
    } else if (col + 1 < land[0].length && land[row][col + 1] == 1){
      // System.out.println(String.format("Right - [%d, %d]", row, col));
      return bottomRightCorner(land, farmCoords, row, col + 1);
    }
    farmCoords[2] = row;
    farmCoords[3] = col;
    // System.out.println();
    if (farmCoords[0] < farmCoords[2])
      fillRemainder(land, row - 1, col);
    // for (int[] r : land) {
      // System.out.println(Arrays.toString(r));
    // }
    return farmCoords;
  }

  static void fillRemainder(int[][] land, int row, int col){
    if (row - 1 >= 0 && land[row - 1][col] == 1) {
      // System.out.println(String.format("Up - [%d, %d]", row, col));
      fillRemainder(land, row - 1, col);
    }
    if (col - 1 >= 0 && land[row][col - 1] == 1){
      // System.out.println(String.format("Left - [%d, %d]", row, col));
      fillRemainder(land, row, col - 1);
    }
    land[row][col] = 0;
  }
}