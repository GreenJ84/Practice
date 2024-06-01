// You are given a string s. The score of a string is defined as the sum of the absolute difference between the ASCII values of adjacent characters.

// Return the score of s.

public class ScoreOfAString {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testScoreOfAString(1, "hello", 13, solution);
    testScoreOfAString(2, "zaz", 50, solution);
  }

  public static void testScoreOfAString(int testNum, String input, int expected, Solution s) {
    int result = s.scoreOfString(input);

    System.out.println( String.format(
      "Test #%d: %d / %d (%s)",
      testNum,
      result,
      expected,
      result == expected? "PASS" : "FAIL"
    ));
  }
}

class Solution {
  public int scoreOfString(String s) {
    int sum = 0;
    for (int idx = 1; idx < s.length(); idx++) {
      sum += Math.abs(s.charAt(idx) - s.charAt(idx-1));
    }
    return sum;
  }
}