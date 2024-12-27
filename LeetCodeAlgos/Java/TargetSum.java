// You are given an integer array nums and an integer target.

// You want to build an expression out of nums by adding one of the symbols '+' and '-' before each integer in nums and then concatenate all the integers.

// For example, if nums = [2, 1], you can add a '+' before 2 and a '-' before 1 and concatenate them to build the expression "+2-1".
// Return the number of different expressions that you can build, which evaluates to target.


class TargetSum {
  public static void main(String[] args) {
    TargetSum obj = new TargetSum();
    System.out.println(obj.findTargetSumWays(new int[]{1, 1, 1, 1, 1}, 3)); // 5
    System.out.println(obj.findTargetSumWays(new int[]{1}, 1)); // 1
  }

  public int findTargetSumWays(int[] nums, int target) {
    return helper(nums, target, 0, 0);
  }

  private int helper(int[] nums, int target, int sum, int idx) {
    if (idx >= nums.length) return sum == target ? 1 : 0;

    return helper(nums, target, sum - nums[idx], idx + 1) + helper(nums, target, sum + nums[idx], idx + 1);
  }
}