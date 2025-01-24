// Given an integer array nums and an integer k, return true if there are two distinct indices i and j in the array such that nums[i] == nums[j] and abs(i - j) <= k.

import java.util.*;

public class ContainsDuplicate2 {
  public static void main(String[] args) {
    ContainsDuplicate2 obj = new ContainsDuplicate2();

    testContainsNearbyDuplicate(1, new int[]{1,2,3,1}, 3, true, obj);
    testContainsNearbyDuplicate(2, new int[]{1,0,1,1}, 1, true, obj);
    testContainsNearbyDuplicate(3, new int[]{1,2,3,1,2,3}, 2, false, obj);
    testContainsNearbyDuplicate(4, new int[]{1,0,3,4}, 2, false, obj);
  }

  public static void testContainsNearbyDuplicate(int testNum, int[] nums, int k, boolean expected, ContainsDuplicate2 obj) {
    boolean result = obj.containsNearbyDuplicate(nums, k);

    System.out.println(String.format(
      "Test %d: %b / %b (%b)",
      testNum,
      result,
      expected,
      result == expected? "PASS" : "FAIL"
    ));
  }

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