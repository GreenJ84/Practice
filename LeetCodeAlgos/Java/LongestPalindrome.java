// Given a string s which consists of lowercase or uppercase letters, return the length of the longest  palindrome that can be built with those letters.

// Letters are case sensitive, for example, "Aa" is not considered a palindrome.

import java.util.HashMap;

public class LongestPalindrome {
  public static void main(String[] args) {
    Solution s = new Solution();

    testLongestPalindrome(1, "abccccdd", 7, s);
    testLongestPalindrome(2, "a", 1, s);
    testLongestPalindrome(3, "ccc", 3, s);
  }

  public static void testLongestPalindrome(int testNum, String s, int expected, Solution sol) {
    int result = sol.longestPalindrome(s);

    System.out.println(String.format(
      "Test %d: %d / %d (%s)",
      testNum,
      result,
      expected,
      result == expected ? "PASS" : "FAIL"
    ));
  }
}

class Solution {
  public int longestPalindrome(String s) {
    HashMap<Character, Integer> count = new HashMap<Character, Integer>();
    for (int i = 0; i < s.length(); i++) {
      char ch = s.charAt(i);
      if (count.containsKey(ch)) {
        count.put(ch, count.get(ch) + 1);
      } else {
        count.put(ch, 1);
      }
    }
    int len = 0;
    for (Integer val : count.values()){
      len += val / 2 * 2;
      if (len % 2 == 0 && val % 2 == 1) {
        len++;
      }
    }
    return len;
  }
}