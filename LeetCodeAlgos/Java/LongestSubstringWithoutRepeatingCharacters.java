// Given a string s, find the length of the longest substring without repeating characters.

import java.util.*;

public class LongestSubstringWithoutRepeatingCharacters {
  public static void main(String[] args) {
    Solution s = new Solution();
    testLengthOfLongestSubstring(1, "abcabcbb", 3, s);
    testLengthOfLongestSubstring(2, "bbbbb", 1, s);
    testLengthOfLongestSubstring(3, "pwwkew", 3, s);
    testLengthOfLongestSubstring(4, "tmmzuxt", 5, s);
    testLengthOfLongestSubstring(5, "", 0, s);
  }

  public static void testLengthOfLongestSubstring(int testNum, String str, int expected, Solution s) {
    int result = s.lengthOfLongestSubstring(str);

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
  public int lengthOfLongestSubstring(String s) {
    int result = 0;
    LinkedHashSet<Character> window = new LinkedHashSet<Character>();

    for (int idx = 0; idx < s.length(); idx++) {
      Character curr = s.charAt(idx);

      System.out.println(String.format(
        "Window: %s (%d - %d)[%b]",
        window.toString(),
        window.size(),
        result,
        window.contains(curr)
      ));

      if (window.contains(curr)){
        if (window.size() > result) result = window.size();
        Iterator<Character> iter = window.iterator();
        while (iter.hasNext() && iter.next() != curr) {
          iter.remove();
        }
        iter.remove();
      }
      window.add(curr);
    }

    return Math.max(result, window.size());
  }
}