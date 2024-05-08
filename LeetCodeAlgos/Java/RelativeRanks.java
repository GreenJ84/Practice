// You are given an integer array score of size n, where score[i] is the score of the ith athlete in a competition. All the scores are guaranteed to be unique.

// The athletes are placed based on their scores, where the 1st place athlete has the highest score, the 2nd place athlete has the 2nd highest score, and so on. The placement of each athlete determines their rank:

// The 1st place athlete's rank is "Gold Medal".
// The 2nd place athlete's rank is "Silver Medal".
// The 3rd place athlete's rank is "Bronze Medal".
// For the 4th place to the nth place athlete, their rank is their placement number (i.e., the xth place athlete's rank is "x").
// Return an array answer of size n where answer[i] is the rank of the ith athlete.

import java.util.*;

public class RelativeRanks {
  public static void main(String[] args){
    Solution s = new Solution();
    testFindRelativeRanks(1, new int[]{5, 4, 3, 2, 1}, new String[]{"Gold Medal", "Silver Medal", "Bronze Medal","4","5"}, s);
    testFindRelativeRanks(2, new int[]{10, 3, 8, 9, 4}, new String[]{"Gold Medal","5","Bronze Medal","Silver Medal","4"}, s);
    testFindRelativeRanks(3, new int[]{1, 2, 3, 4, 5}, new String[]{"5", "4", "Bronze Medal","Silver Medal","Gold Medal"}, s);
  }

  static void testFindRelativeRanks(int testNum , int[] score, String[] expected, Solution s){
    String[] result = s.findRelativeRanks(score);

    System.out.println(
      String.format(
        "Test #%d: %s / %s (%b)",
        testNum,
        Arrays.toString(result),
        Arrays.toString(expected),
        Arrays.equals(result, expected)
      )
    );
    assert (Arrays.equals(result, expected));
  }
}

class Solution {
  public String[] findRelativeRanks(int[] score) {
      SortedMap<Integer, Integer> map = new TreeMap<Integer, Integer>();
      for (int i = 0; i < score.length; i++) {
          map.put(score[i], i);
      }
      String[] result = new String[score.length];
      int pos = score.length;
      for (int idx : map.values()) {
        switch (pos) {
          case 1:
            result[idx] = new String("Gold Medal");
            break;
          case 2:
            result[idx] =  new String("Silver Medal");
            break;
          case 3:
            result[idx] =  new String("Bronze Medal");
            break;
          default:
            result[idx] =  new String(Integer.toString(pos));
            break;
        }
        pos--;
      }
      return result;
  }
}