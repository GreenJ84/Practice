// You are given a 0-indexed integer array nums. A subarray of nums is called continuous if:

// Let i, i + 1, ..., j be the indices in the subarray. Then, for each pair of indices i <= i1, i2 <= j, 0 <= |nums[i1] - nums[i2]| <= 2.
// Return the total number of continuous subarrays.

// A subarray is a contiguous non-empty sequence of elements within an array.

import java.util.TreeMap;

public class ContinuousSubarrays {
  public static void main(String[] args) {
    ContinuousSubarrays oj = new ContinuousSubarrays();

    System.out.println(oj.continuousSubarrays(new int[]{5,4,2,4})); // 8
    System.out.println(oj.continuousSubarrays(new int[]{1,2,3})); // 6
    System.out.println(oj.continuousSubarrays(new int[]{3,9,2,8,1,7})); // 6
  }

  public long continuousSubarrays(int[] nums) {
    int n = nums.length;
    long ans = 0;
    int start = 0;

    TreeMap<Integer, Integer> freqMap = new TreeMap<>();
    for (int end = 0; end < n; end++){
      freqMap.put(nums[end], freqMap.getOrDefault(nums[end], 0) + 1);
      while (freqMap.lastKey() - freqMap.firstKey() > 2) {
        freqMap.put(nums[start], freqMap.get(nums[start]) - 1);
        if (freqMap.get(nums[start]) == 0) freqMap.remove(nums[start]);
        start++;
      }
      ans += end - start + 1;
    }

    return ans;
  }
}
