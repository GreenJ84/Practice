import java.util.Arrays;

public class SpecialArrayWithXElementsGreaterThanOrEqualX {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testSpecialArray(1, new int[]{3,5}, 2, solution);
    testSpecialArray(2, new int[]{0,0}, -1, solution);
    testSpecialArray(3, new int[]{0,4,3,0,4}, 3, solution);
    testSpecialArray(4, new int[]{3,6,7,7,0}, -1, solution);
    testSpecialArray(5, new int[] {0,1,5,6,10,9,7,0,1,2,4,3}, 5, solution);
    testSpecialArray(5, new int[] {0,1,5,6,10,9,7,0,1,2,5,3}, -1, solution);
    testSpecialArray(5, new int[] {100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100}, 100, solution);
  }

  public static void testSpecialArray(int testNum, int[] nums, int expected, Solution s){
    int result = s.specialArray(nums);

    System.out.println(String.format(
      "Test %d: %d / %d (%s)",
      testNum,
      result,
      expected,
      result == expected ? "PASS" : "FAIL"
    ));
  }
}

class Solution {
  public int specialArray(int[] nums) {
    Arrays.sort(nums); // Sort asc
    // Automatically return if smallest GTE Array length
    if (nums[0] >= nums.length){ return nums.length; }
    // Automatically return if cant start window
    if (nums[nums.length - 1] == 0){ return -1; }
    // Check largest to smallest:
    int res = 1;
    for (int idx = nums.length - 1; idx >= 0; idx--){
      // IF GTE current window -> continue expanding
      if (nums[idx] >= res){
        res++;
      }
      // IF LT current window:
      // Check value equality to last step for failures
      else {
        return nums[idx] == res - 1 ? -1 : --res;
      }
    }
    // Java compiler failsafe for full Array traversal
    return --res;
  }
}