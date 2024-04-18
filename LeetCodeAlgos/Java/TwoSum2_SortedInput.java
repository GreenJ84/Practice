// Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number. Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 <= numbers.length.
// Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.

// The tests are generated such that there is exactly one solution. You may not use the same element twice.
// Your solution must use only constant extra space.

public class TwoSum2_SortedInput {
  public static void main(String[] args) {
    Solution s = new Solution();

    int[] test1 = new int[]{2, 7, 11, 15};
    int target = 9;
    int[] expected1 = new int[]{1, 2};
    testTwoSum(1, test1, target, expected1, s);

    int[] test2 = new int[]{2, 3, 4};
    int target2 = 6;
    int[] expected2 = new int[]{1, 3};
    testTwoSum(2, test2, target2, expected2, s);

    int[] test3 = new int[]{-1, 0};
    int target3 = -1;
    int[] expected3 = new int[]{1, 2};
    testTwoSum(3, test3, target3, expected3, s);
  }

  static void testTwoSum(int testNum, int[] input, int target, int[] expected, Solution s) {
    int[] result = s.twoSum(input, target);

    System.out.println(
      String.format(
        "Test #%d: [%d, %d] / [%d, %d] (%b)",
        testNum,
        result[0],
        result[1],
        expected[0],
        expected[1],
        result[0] == expected[0] && result[1] == expected[1]
      ));
  }
}

class Solution {
  public int[] twoSum(int[] numbers, int target) {
    int n = numbers.length;
    for (int left = 0; left < n; left++) {
      if (numbers[left] + numbers[n-1] < target) continue;
      for (int right = left + 1; right < n; right++) {
        if (numbers[left] + numbers[right] == target) {
          return new int[]{left + 1, right + 1};
        }
        else if (numbers[left] + numbers[right] > target) break;
      }
    }
    return new int[]{};
  }
}