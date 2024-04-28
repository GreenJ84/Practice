import java.util.*;

public class GasStation {
  public static void main(String[] args) {
    Solution s = new Solution();

    testCanCompleteCircuit(1, new int[]{1,2,3,4,5}, new int[]{3,4,5,1,2}, 3, s);

    testCanCompleteCircuit(2, new int[]{2,3,4}, new int[]{3,4,3}, -1, s);
  }

  static void testCanCompleteCircuit(int tesTNum, int[] gas, int[] cost, int expected,  Solution s) {
      int result = s.canCompleteCircuit(gas, cost);

      System.out.println(String.format(
        "Test %d: %d / %d (%b)",
        tesTNum,
        result,
        expected,
        result == expected
      ));
  }
}

class Solution {
  public int canCompleteCircuit(int[] gas, int[] cost) {
      if (Arrays.stream(gas).sum() < Arrays.stream(cost).sum()) {
        return -1;
      }
      int tank = 0;
      int start = 0;
      for (int i = 0; i < gas.length; i++) {
        tank += gas[i] - cost[i];
        if (tank < 0) {
          start = i + 1;
          tank = 0;
        }
      }
      return start;
  }
}