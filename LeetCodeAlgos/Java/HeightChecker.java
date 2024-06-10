// school is trying to take an annual photo of all the students. The students are asked to stand in a single file line in non-decreasing order by height. Let this ordering be represented by the integer array expected where expected[i] is the expected height of the ith student in line.
// You are given an integer array heights representing the current order that the students are standing in. Each heights[i] is the height of the ith student in line (0-indexed).
// Return the number of indices where heights[i] != expected[i].

import java.util.Arrays;

public class HeightChecker {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testHeightChecker(1, new int[]{1,1,4,2,1,3}, 3, solution);
    testHeightChecker(2, new int[]{5,1,2,3,4}, 5, solution);
    testHeightChecker(3, new int[]{1,2,3,4,5}, 0, solution);

  }

  private static void testHeightChecker(int testNum, int[] heights, int expected, Solution s) {
    int result = s.heightChecker(heights);

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
  public int heightChecker(int[] heights) {
    int[] sorted = heights.clone();
    Arrays.sort(sorted);

    int misplaced = 0;
    for (int i = 0; i < heights.length; i++) {
      if (heights[i]!= sorted[i]) misplaced++;
    }
    return misplaced;
  }
}