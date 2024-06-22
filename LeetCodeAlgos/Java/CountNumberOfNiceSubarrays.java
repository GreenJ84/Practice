// Given an array of integers nums and an integer k. A continuous subarray is called nice if there are k odd numbers on it.

// Return the number of nice sub-arrays.


public class CountNumberOfNiceSubarrays {
  public static void main(String[] args) {
    Solution s = new Solution();

    testNumberOfSubarrays(1, new int[]{1,1,2,1,1}, 3, 2, s);
    testNumberOfSubarrays(2, new int[]{2,4,6}, 1, 0, s);
    testNumberOfSubarrays(3, new int[]{2,2,2,1,2,2,1,2,2,2}, 2, 16, s);
  }

  public static void testNumberOfSubarrays(int testNum, int[] nums, int k, int expected, Solution s) {
    int result = s.numberOfSubarrays(nums, k);

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
  public int numberOfSubarrays(int[] nums, int k) {
      int[] prefixCount = new int[nums.length + 1];
      prefixCount[0] = 1;

      int sum = 0;
      int niceArrays = 0;
      for (int num : nums) {
          sum += num % 2;
          if (sum >= k) {
              niceArrays += prefixCount[sum - k];
          }
          prefixCount[sum]++;
      }
      
      return niceArrays;
  }
}