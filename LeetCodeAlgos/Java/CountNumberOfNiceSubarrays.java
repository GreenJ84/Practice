// Given an array of integers nums and an integer k. A continuous subarray is called nice if there are k odd numbers on it.

// Return the number of nice sub-arrays.


public class CountNumberOfNiceSubarrays {
  public static void main(String[] args) {
    CountNumberOfNiceSubarrays obj = new CountNumberOfNiceSubarrays();

    testNumberOfSubarrays(1, new int[]{1,1,2,1,1}, 3, 2, obj);
    testNumberOfSubarrays(2, new int[]{2,4,6}, 1, 0, obj);
    testNumberOfSubarrays(3, new int[]{2,2,2,1,2,2,1,2,2,2}, 2, 16, obj);
  }

  public static void testNumberOfSubarrays(int testNum, int[] nums, int k, int expected, CountNumberOfNiceSubarrays obj) {
    int result = obj.numberOfSubarrays(nums, k);

    System.out.println(String.format(
      "Test %d: %d / %d (%s)",
      testNum,
      result,
      expected,
      result == expected? "PASS" : "FAIL"
    ));
  }

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