// You are given two strings s and t consisting of only lowercase English letters.

// Return the minimum number of characters that need to be appended to the end of s so that t becomes a subsequence of s.

// A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.

public class AppendCharactersToStringToMakeSubsequence {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testAppendCharacters(1,  "coaching", "coding", 4, solution);
    testAppendCharacters(2,  "abcde", "a", 0, solution);
    testAppendCharacters(3,  "a", "a", 0, solution);
  }

  public static void testAppendCharacters(int testNum, String s, String t, int expected, Solution sol) {
    int result = sol.appendCharacters(s, t);

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
  public int appendCharacters(String s, String t) {
    int tIdx = 0;
    char ch = t.charAt(tIdx);
    for (int sIdx = 0; sIdx < s.length(); sIdx++) {
      if (s.charAt(sIdx) == ch) {
        tIdx++;
        if (tIdx == t.length()) return 0;
        ch = t.charAt(tIdx);
      }
    }
    return t.length() - tIdx;
  }
}