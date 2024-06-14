// You are given an integer array nums. In one move, you can pick an index i where 0 <= i < nums.length and increment nums[i] by 1.
// Return the minimum number of moves to make every value in nums unique.
// The test cases are generated so that the answer fits in a 32-bit integer.

import java.util.*;

public class MinimumIncrementToMakeArrayUnique {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testMinIncrementForUnique(1, new int[]{1,2,2}, 1, solution);
    testMinIncrementForUnique(2, new int[]{3,2,1,2,1,7}, 6, solution);
  }

  private static void testMinIncrementForUnique(int testNum, int[] nums, int expected, Solution s) {
    int result = s.minIncrementForUnique(nums);

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
  public int minIncrementForUnique(int[] nums) {
      Arrays.sort(nums);
      int increment = 0;
      int prev = nums[0];
      for (int i = 1; i < nums.length; i++) {
        if (nums[i] <= prev){
          prev++;
          increment += prev - nums[i];
        } else {
          prev = nums[i];
        }
      }
      return increment;
  }
}