public class ClimbingStairs {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testClimbStairs(1, 2, 2, solution);
    testClimbStairs(2, 3, 3, solution);
    testClimbStairs(3, 4, 5, solution);
    testClimbStairs(4, 5, 8, solution);
  }

  private static void testClimbStairs(int testNum, int n, int expected, Solution s){
    int result = s.climbStairs(n);

    System.out.println(String.format(
      "Test %d: %d / %d (%b)",
      testNum,
      result,
      expected,
      result == expected
    ));
  }
}

class Solution {
  public int climbStairs(int n) {
      if (n < 3) return n;

      int firstStep = 1;
      int secondStep = 2;
      for (int i = 3; i <= n; i++){
          int temp = firstStep;
          firstStep = secondStep;
          secondStep += temp;
      }
      return secondStep;
  }
}