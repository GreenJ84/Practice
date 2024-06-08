// Given an integer array nums and an integer k, return true if nums has a good subarray or false otherwise.
// A good subarray is a subarray where:
  // its length is at least two, and
  // the sum of the elements of the subarray is a multiple of k.
// Note that:
  // A subarray is a contiguous part of the array.
  // An integer x is a multiple of k if there exists an integer n such that x = n * k. 0 is always a multiple of k.

import java.util.Map;
import java.util.HashMap;
public class ContinuousSubarraySum {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testcheckSubarraySum(1, new int[]{23,2,4,6,7}, 6, true, solution);
    testcheckSubarraySum(2, new int[]{23,2,6,4,7}, 6, true, solution);
    testcheckSubarraySum(3, new int[]{23,2,6,4,7}, 13, false, solution);
  }

  private static void testcheckSubarraySum(int testNum, int[] nums, int k, boolean expected, Solution s){
    boolean found = s.checkSubarraySum(nums, k);

    System.out.println(String.format(
      "Test %d: %b / %b (%s)",
      testNum,
      found,
      expected,
      found == expected? "PASS" : "FAIL"
    ));
  }
}

//! Brute Force!
// class Solution {
//   public boolean checkSubarraySum(int[] nums, int k) {
//     long windowSum;

//     for (int start = 0; start < nums.length - 1; start++) {
//       windowSum = (long)nums[start];
//       for (int run = start + 1; run < nums.length; run++) {
//         windowSum += (long)nums[run];
//         if (windowSum % k == 0) return true;
//       }
//     }
//     return false;
//   }
// }

class Solution {
  public boolean checkSubarraySum(int[] nums, int k) {
      Map<Integer, Integer> remainders = new HashMap<>();
      remainders.put(0, -1);

      int sum = 0;
      for (int idx = 0; idx < nums.length; idx++) {
          sum += nums[idx];
          int remainder = k == 0 ? sum : sum % k;

          if (remainders.containsKey(remainder)) {
              // Check if segment length is at least two
              if (idx - remainders.get(remainder) > 1) {
                  return true;
              }
          } else {
              remainders.put(remainder, idx);
          }
      }
      return false;
  }
}