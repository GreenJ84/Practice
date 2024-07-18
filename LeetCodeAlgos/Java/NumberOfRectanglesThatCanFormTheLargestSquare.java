// You are given an array rectangles where rectangles[i] = [li, wi] represents the ith rectangle of length li and width wi.

// You can cut the ith rectangle to form a square with a side length of k if both k <= li and k <= wi. For example, if you have a rectangle [4,6], you can cut it to get a square with a side length of at most 4.

// Let maxLen be the side length of the largest square you can obtain from any of the given rectangles.

// Return the number of rectangles that can make a square with a side length of maxLen.

public class NumberOfRectanglesThatCanFormTheLargestSquare {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testCountGoodRectangles(1, new int[][]{{5,8},{3,9},{5,12},{16,5}}, 3, solution);

    testCountGoodRectangles(2, new int[][]{{2,3},{3,7},{4,3},{3,7}}, 3, solution);
  }

  public static void testCountGoodRectangles(int testNum, int[][] rectangles, int expected, Solution s) {
    int result = s.countGoodRectangles(rectangles);

    System.out.println(String.format(
      "Test %d: %d / %d (%s)",
      testNum,
      result,
      expected,
      result == expected? "PASS" : "FAIL"
    ));
  }
}

class Solution {
  public int countGoodRectangles(int[][] rectangles) {
      int maxSquare = 0, count = 0;

      for (int[] rectangle : rectangles) {
        int length = Math.min(rectangle[0], rectangle[1]);
        if (length > maxSquare) {
          maxSquare = length;
          count = 1;
        } else if (length == maxSquare) {
          count++;
        }
      }
      System.out.println(maxSquare);
      return count;
  }
}
