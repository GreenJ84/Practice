// You are given a 0-indexed array of integers nums of length n. You are initially positioned at nums[0].
// Each element nums[i] represents the maximum length of a forward jump from index i. In other words, if you are at nums[i], you can jump to any nums[i + j] where:
  // 0 <= j <= nums[i] and
  // i + j < n
// Return the minimum number of jumps to reach nums[n - 1]. The test cases are generated such that you can reach nums[n - 1].

public class Solution {
     public int Jump(int[] nums)
    {
        int jumps = 0, end = 0, lastJump = 0;
        for (int i = 0; i < nums.Length - 1; i++)
        {
            lastJump = Math.Max(lastJump, i + nums[i]);
            if (i == end)
            {
                jumps++;
                end = lastJump;
            }
        }
        return jumps;
    }
    // public int Jump(int[] nums) {
    //     int end = nums.Length - 1;
    //     int jumps = 0;
    //     while (end != 0)
    //     {
    //       int lastJump = end;
    //       for(int idx = end - 1; idx > -1; idx--){
    //         if (idx + nums[idx] == end && idx < lastJump)
    //           lastJump = idx;
    //       }
    //       end = lastJump;
    //       jumps++;
    //     }
    //     return jumps;
    // }
}