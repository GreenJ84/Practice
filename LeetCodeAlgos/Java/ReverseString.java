// Write a function that reverses a string. The input string is given as an array of characters s.

// You must do this by modifying the input array in-place with O(1) extra memory.

import java.util.Arrays;

public class ReverseString {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testReverseString(1, new char[]{'h','e','l','l','o'}, new char[]{'o','l','l','e','h'}, solution);

    testReverseString(2, new char[]{'H','a','n','n','a','h'}, new char[]{'h','a','n','n','a','H'}, solution);
  }

  public static void testReverseString(int testNum, char[] input, char[] expected, Solution s) {
    s.reverseString(input);

    System.out.println( String.format(
      "Test #%d: %s / %s (%s)",
      testNum,
      Arrays.toString(input),
      Arrays.toString(expected),
      Arrays.equals(input, expected)? "PASS" : "FAIL"
    ));
  }
}

class Solution {
  public void reverseString(char[] s) {
    for (int idx = 0; idx <= s.length / 2 - 1; idx++){
      char temp = s[idx];
      s[idx] = s[s.length - 1 - idx];
      s[s.length - 1 - idx] = temp;
    }
  }
}