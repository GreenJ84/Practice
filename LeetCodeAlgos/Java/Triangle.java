// Given a triangle array, return the minimum path sum from top to bottom.

// For each step, you may move to an adjacent number of the row below. More formally, if you are on index i on the current row, you may move to either index i or index i + 1 on the next row.

import java.util.*;

public class Triangle {
  public static void main(String[] args) {
    Solution solution = new Solution();

    // List.of provides an Immutable list but mutability is nice....
    testMinimumTotal(1, mutableList(List.of(
      List.of(2),
      List.of(3,4),
      List.of(6,5,7),
      List.of(4,1,8,3)
    )), 11, solution);
    testMinimumTotal(2, mutableList(List.of(
      List.of(-10)
    )), -10, solution);
  }

  private static List<List<Integer>> mutableList(List<List<Integer>> list) {
    List<List<Integer>> dp = new ArrayList<>();
    for (List<Integer> row : list) {
        dp.add(new ArrayList<>(row));
    }
    return dp;
  }

  private static void testMinimumTotal(int testNum, List<List<Integer>> triangle, int expected, Solution s) {
    int result = s.minimumTotal(triangle);

    System.out.println(String.format(
      "Test %d: %d / %d (%s)",
      testNum,
      result,
      expected,
      result == expected
    ));
  }
}

class Solution {
  public int minimumTotal(List<List<Integer>> triangle) {
    if (triangle == null || triangle.size() == 0) throw new IllegalArgumentException("A Triangle of at least one row required.");
    if (triangle.size() == 1) return triangle.get(0).get(0);

    int n = triangle.size();
    for (int row = n - 2; row >= 0; row--){
      List<Integer> curr = triangle.get(row);
      List<Integer> below = triangle.get(row + 1);
      for (int idx = 0; idx < curr.size(); idx++){
        curr.set(idx, curr.get(idx) + Math.min(below.get(idx), below.get(idx + 1)));
      }
    }
    return triangle.get(0).get(0);
  }
}