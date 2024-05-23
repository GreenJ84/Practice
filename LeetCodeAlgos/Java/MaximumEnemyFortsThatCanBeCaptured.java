public class MaximumEnemyFortsThatCanBeCaptured {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testCaptureForts(0, new int[] {1,0,0,-1,0,0,0,0,1}, 4, solution);
    testCaptureForts(0, new int[]{0,0,1,-1}, 0, solution);
  }

  public static void testCaptureForts(int testNum, int[] forts, int expected, Solution s) {
    int result = s.captureForts(forts);

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
  public int captureForts(int[] forts) {
    int ltr = 0;
    int runSum = 0;
    boolean moving = false;
    for (int i = 0; i < forts.length; i++) {
      if (forts[i] == 1) {
        moving = true;
        runSum = 0;
      } else if (forts[i] == 0 && moving) {
        runSum++;
      } else if (forts[i] == -1 && moving) {
        ltr = Math.max(ltr, runSum);
        moving = false;
      }
    }
    runSum = 0;
    int rtl = 0;
    for (int i =  forts.length - 1; i >= 0; i--) {
      if (forts[i] == 1) {
        moving = true;
        runSum = 0;
      } else if (forts[i] == 0 && moving) {
        runSum++;
      } else if (forts[i] == -1 && moving) {
        rtl = Math.max(rtl, runSum);
        moving = false;
      }
    }
    return Math.max(ltr, rtl);
  }
}