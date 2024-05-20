// Given an integer array nums and an integer k, return true if there are two distinct indices i and j in the array such that nums[i] == nums[j] and abs(i - j) <= k.

import java.util.*;

public class ContainsDuplicate2 {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testContainsNearbyDuplicate(1, new int[]{1,2,3,1}, 3, true, solution);
    testContainsNearbyDuplicate(2, new int[]{1,0,1,1}, 1, true, solution);
    testContainsNearbyDuplicate(3, new int[]{1,2,3,1,2,3}, 2, false, solution);
    testContainsNearbyDuplicate(4, new int[]{1,0,3,4}, 2, false, solution);
  }

  public static void testContainsNearbyDuplicate(int testNum, int[] nums, int k, boolean expected, Solution s) {
    boolean result = s.containsNearbyDuplicate(nums, k);

    System.out.println(String.format(
      "Test %d: %b / %b (%b)",
      testNum,
      result,
      expected,
      result == expected? "PASS" : "FAIL"
    ));
  }
}


class Solution {
  public boolean containsNearbyDuplicate(int[] nums, int k) {
    HashMap<Integer, Integer> map = new HashMap<Integer, Integer>();
    for (int idx = 0; idx < nums.length; idx++) {
      if (map.containsKey(nums[idx])){
        if (Math.abs(map.get(nums[idx]) - idx) <= k) return true;
        map.put(nums[idx], idx);
      } else {
        map.put(nums[idx], idx);
      }
    }
    return false;
  }
}