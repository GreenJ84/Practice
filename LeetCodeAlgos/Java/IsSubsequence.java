// Given two strings s and t, return true if s is a subsequence of t, or false otherwise.

// A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not).

class Solution {
  public boolean isSubsequence(String s, String t) {
    if (s.length() > t.length()) {
      return false;
    } else if (s.length() == t.length()) {
      return s.equals(t);
    } else if (s.length() == 0) {
      return true;
    }
    for (int start = 0; start < t.length() - s.length() + 1; start++) {
      if (t.charAt(start) == s.charAt(0)) {
        if (s.length() == 1) {
          return true;
        }
        System.out.println(String.format("%c - %d - %c", t.charAt(start), 0, s.charAt(0)));

        int idx = 1;
        for (int run = 1; start + run < t.length(); run++) {
          System.out.println(String.format("%c - %d - %c", t.charAt(start + run), idx, s.charAt(idx)));
          if (t.charAt(start + run) == s.charAt(idx)) {
            idx++;
          }
          if (idx == s.length()) {
            return true;
          }
        }
      }
    }
    return false;
  }
}

class IsSubsequence{
  public static void main(String[] args) {
    Solution sol = new Solution();

    testIsSubsequence(1, "abc", "ahbgdc", true, sol);
    testIsSubsequence(1, "axc", "ahbgdc", false, sol);
  }

  public static void testIsSubsequence(int testNum, String s, String t, boolean expected, Solution sol) {
    boolean result = sol.isSubsequence(s, t);

    System.out.println(String.format("Test %d: %s -> %s\n %b / %b", testNum, s, t, result, expected));
  }
}