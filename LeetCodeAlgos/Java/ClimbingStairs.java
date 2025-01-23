public class ClimbingStairs {
  public static void main(String[] args) {
    ClimbingStairs obj = new ClimbingStairs();

    testClimbStairs(1, 2, 2, obj);
    testClimbStairs(2, 3, 3, obj);
    testClimbStairs(3, 4, 5, obj);
    testClimbStairs(4, 5, 8, obj);
  }

  private static void testClimbStairs(int testNum, int n, int expected, ClimbingStairs obj){
  int result = obj.climbStairs(n);

    System.out.println(String.format(
      "Test %d: %d / %d (%b)",
      testNum,
      result,
      expected,
      result == expected
    ));
  }

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
