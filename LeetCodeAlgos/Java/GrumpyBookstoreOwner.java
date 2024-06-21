// There is a bookstore owner that has a store open for n minutes. Every minute, some number of customers enter the store. You are given an integer array customers of length n where customers[i] is the number of the customer that enters the store at the start of the ith minute and all those customers leave after the end of that minute.

// On some minutes, the bookstore owner is grumpy. You are given a binary array grumpy where grumpy[i] is 1 if the bookstore owner is grumpy during the ith minute, and is 0 otherwise.

// When the bookstore owner is grumpy, the customers of that minute are not satisfied, otherwise, they are satisfied.

// The bookstore owner knows a secret technique to keep themselves not grumpy for minutes consecutive minutes, but can only use it once.

// Return the maximum number of customers that can be satisfied throughout the day.

import java.util.*;

public class GrumpyBookstoreOwner {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testMaxSatisfied(1, new int[]{1,0,1,2,1,1,7,5}, new int[]{0,1,0,1,0,1,0,1}, 3, 16, solution);
    testMaxSatisfied(2, new int[]{1}, new int[]{0}, 1, 1, solution);
  }

  private static void testMaxSatisfied(int testNum, int[] customers, int[] grumpy, int minutes, int expected, Solution s) {
    int result = s.maxSatisfied(customers, grumpy, minutes);

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
  public int maxSatisfied(int[] customers, int[] grumpy, int minutes) {
    int notGrumpy = 0;
    List<Integer> grumps = new ArrayList<>();
    for (int idx = 0; idx < customers.length; idx++) {
      if (grumpy[idx] == 0) {
        notGrumpy += customers[idx];
      }
      else {
        grumps.add(idx);
      }
    }
    if (grumps.size() == 0) return notGrumpy;

    int end = 0;
    int maxSpec = customers[grumps.get(end)];
    int runningSpec = maxSpec;
    for (int start = 1; start < grumps.size(); start++) {
      int curIdx = grumps.get(start);
      while (end < start && curIdx - grumps.get(end) >= minutes){
        runningSpec -= customers[grumps.get(end)];
        end++;
      }
      runningSpec += customers[grumps.get(start)];
      maxSpec = Math.max(maxSpec, runningSpec);
    }
    return notGrumpy + maxSpec;
  }
}