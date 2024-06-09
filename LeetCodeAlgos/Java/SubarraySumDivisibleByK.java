
// Given an integer array nums and an integer k, return the number of non-empty subarrays that have a sum divisible by k.

// A subarray is a contiguous part of an array.

import java.util.HashMap;

public class SubarraySumDivisibleByK {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testSubarraysDivByK(1, new int[]{4,5,0,-2,-3,1}, 5, 7, solution);
    testSubarraysDivByK(2, new int[]{5}, 9, 0, solution);
  }

  private static void testSubarraysDivByK(int testNum, int []nums, int k, int expected, Solution s){
    int result = s.subarraysDivByK(nums, k);

    System.out.println(String.format(
      "Test %d: %d / %d (%s)",
      testNum,
      result,
      expected,
      result == expected? "PASS" : "FAIL"
    ));
  }
}

// class Solution {
//   public int subarraysDivByK(int[] nums, int k) {
//     int count = 0;
//     int windowSum;
//     for (int start = 0; start < nums.length; start++) {
//       windowSum = 0;
//       for (int idx = start; idx < nums.length; idx++) {
//         windowSum += nums[idx];
//         if (windowSum % k == 0) count++;
//       }
//     }
//     return count;
//   }
// }

class Solution {
  public int subarraysDivByK(int[] nums, int k) {
    int count = 0;
    int prefixSum = 0;
    HashMap<Integer, Integer> remainders = new HashMap<>();
    remainders.put(0, 1);

    for (int num : nums) {
      prefixSum += num;
      int mod = prefixSum % k;
      if (mod < 0) mod += k;

      if (remainders.containsKey(mod)) {
        count += remainders.get(mod);
        remainders.put(mod, remainders.get(mod) + 1);
      } else {
        remainders.put(mod, 1);
      }
    }
    
    return count;
  }
}