// You are given a 0-indexed integer array nums of length n.

// nums contains a valid split at index i if the following are true:

// The sum of the first i + 1 elements is greater than or equal to the sum of the last n - i - 1 elements.
// There is at least one element to the right of i. That is, 0 <= i < n - 1.
// Return the number of valid splits in nums.

public class NumberOfWaysToSplitArray {
  public static void main(String[] args) {
    NumberOfWaysToSplitArray obj = new NumberOfWaysToSplitArray();

    System.out.println(obj.waysToSplitArray(new int[]{10,4,-8,7}));
    System.out.println(obj.waysToSplitArray(new int[]{2,3,1,0}));
  }

  public int waysToSplitArray(int[] nums) {
    int ways = 0;
    long right = 0;
    for (int num : nums){
      right += num;
    }

    long left = 0;
    for (int i = 0; i < nums.length - 1; i++) {
      left += nums[i];
      right -= nums[i];
      if (left >= right) ways++;
    }
    return ways;
  }
}
