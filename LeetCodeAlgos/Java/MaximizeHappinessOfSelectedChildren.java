// You are given an array happiness of length n, and a positive integer k.

// There are n children standing in a queue, where the ith child has happiness value happiness[i]. You want to select k children from these n children in k turns.

// In each turn, when you select a child, the happiness value of all the children that have not been selected till now decreases by 1. Note that the happiness value cannot become negative and gets decremented only if it is positive.

// Return the maximum sum of the happiness values of the selected children you can achieve by selecting k children.

public class MaximizeHappinessOfSelectedChildren {
  public static void main(String[] args) {
    Solution s = new Solution();

    testMaximumHappinessSum(1, new int[]{1,2,3}, 2, 4, s);
    testMaximumHappinessSum(2, new int[]{1,1,1,1}, 2, 1, s);
    testMaximumHappinessSum(3, new int[]{2,3,4,5}, 1, 5, s);


  }

  static void testMaximumHappinessSum(int testNum, int[] happiness, int k, long expected, Solution s) {
    long result = s.maximumHappinessSum(happiness, k);

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
  public long maximumHappinessSum(int[] happiness, int k) {
      java.util.Arrays.sort(happiness);
      long sum = 0;
      int diff = 0;
      for (int i = happiness.length - 1; i >= 0; i--) {
        int curr = happiness[i] - diff;
        if (curr <= 0){
          return sum;
        }
        sum += curr;
        diff++;
        if (diff == k){
          return sum;
        }
      }
      return sum;
  }
}