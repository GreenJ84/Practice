// Given an integer array nums that does not contain any zeros, find the largest positive integer k such that -k also exists in the array.

// Return the positive integer k. If there is no such integer, return -1.

import java.util.*;
public class LargestPositiveIntegerThatExistsWithItsNegative {
  public static void main(String[] args){
    Solution s = new Solution();

    testFindMaxK(1, new int[]{-1,2,-3,3}, 3, s);
    testFindMaxK(2, new int[]{-1,10,6,7,-7,1}, 7, s);
    testFindMaxK(3, new int[]{-10,8,6,7,-2,-3}, -1, s);
  }

  public static void testFindMaxK(int testNum, int[] nums, int expected, Solution s) {
    int result = s.findMaxK(nums);

    System.out.println(String.format(
      "Test %d: %d / %d (%b)",
      testNum,
      result,
      expected,
      result == expected
    ));
    assert result == expected;
  }
}

class Solution {
  public int findMaxK(int[] nums) {
      Arrays.sort(nums);
      System.out.println(Arrays.toString(nums));
      int neg = 0;
      int pos = nums.length - 1;
      while (nums[neg] < 0 && nums[pos] > 0) {
        if (nums[neg] == -nums[pos]) return nums[pos];
        else if (nums[neg] < -nums[pos]) neg++;
        else pos--;
      }
      return -1;
  }
}