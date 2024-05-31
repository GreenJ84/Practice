// Given an integer array nums, in which exactly two elements appear only once and all the other elements appear exactly twice. Find the two elements that appear only once. You can return the answer in any order.
// You must write an algorithm that runs in linear runtime complexity and uses only constant extra space.

import java.util.Arrays;

public class SingleNumber3 {
  public static void main(String[] args) {
    Solution s = new Solution();

    testSingleNumber(1, new int[]{1,2,1,3,2,5}, new int[]{3,5}, s);
    testSingleNumber(0, new int[]{1,0}, new int[]{1,0}, s);
    testSingleNumber(0, new int[]{0,1}, new int[]{0,1}, s);
  }

  public static void testSingleNumber(int testNum, int[] nums, int[] expected, Solution s){
    int[] result = s.singleNumber(nums);

    System.out.println(String.format(
      "Test %d: %s / %s (%b)",
      testNum,
      Arrays.toString(result),
      Arrays.toString(expected),
      Arrays.equals(result, expected)
    ));
  }
}

class Solution {
  public int[] singleNumber(int[] nums) {
    if (nums.length == 2) return nums;
    // Hold XOR of the unique two elements
    int xor = 0;
    // Get the binary XOR of against all nums
    for (int num : nums) {
        xor ^= num;
    }

    // Find the lowest bit of combined xor
    int lowestBit = xor & (-xor);

    int[] result = new int[2];
    // Filter nums via XOR based on the lowest bit
    for (int num : nums) {
        if ((lowestBit & num) == 0) {
            result[0] ^= num;
        } else {
            result[1] ^= num;
        }
    }
    return result;
  }
}