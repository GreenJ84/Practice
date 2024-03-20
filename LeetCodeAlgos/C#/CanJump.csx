// You are given an integer array nums. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position.

// Return true if you can reach the last index, or false otherwise.

public class Solution {
    public bool CanJump(int[] nums) {
      bool[] dp = new bool[nums.Length];
      dp[nums.Length - 1] = true;
      for (int idx = nums.Length - 2; idx >= 0; idx--){
        for (int count = 1; count <= nums[idx]; count++) {
          if (idx + count >= nums.Length){ continue; }
          if (dp[idx + count] == true){
            dp[idx] = true;
          }
        }
      }
      return dp[0];
    }
}

public void TestCanJump(int testNum, int[] nums, bool expected) {
  Solution solution = new Solution();
  bool result = solution.CanJump(nums);
  bool passed = result == expected;

  Console.WriteLine($"Test {testNum}: {result}/{expected} ({passed})");
}

TestCanJump(1, new int[] { 2, 3, 1, 1, 4 }, true);
TestCanJump(2, new int[] { 3, 2, 1, 0, 4 }, false);
TestCanJump(3, new int[] { 0 }, true);
TestCanJump(3, new int[] { 2, 0 }, true);