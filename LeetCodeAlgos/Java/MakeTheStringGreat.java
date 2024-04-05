// Given a string s of lower and upper case English letters.
// A good string is a string which doesn't have two adjacent characters s[i] and s[i + 1] where:
  // 0 <= i <= s.length - 2
  // s[i] is a lower-case letter and s[i + 1] is the same letter but in upper-case or vice-versa.
// To make the string good, you can choose two adjacent characters that make the string bad and remove them. You can keep doing this until the string becomes good.
// Return the string after making it good. The answer is guaranteed to be unique under the given constraints.
// Notice that an empty string is also good.

class Solution {
  public String makeGood(String s) {
      // Not long enough to need checking
      if (s == null || s.length() < 2) {
        return s;
      }
      // A mutable string structure
      StringBuilder sb = new StringBuilder(s);
      // Look at all but last characters and its following value
      int checkIdx = 0;
      while (checkIdx < sb.length() - 1) {
        char first = sb.charAt(checkIdx);
        char second = sb.charAt(checkIdx + 1);
        // Change first if characters aren't same casing
        if (Character.isUpperCase(first) ^ Character.isUpperCase(second)) {
          first = Character.isUpperCase(first) ? Character.toLowerCase(first) : Character.toUpperCase(first);
          // Compare and remove if needed
          if (first == second) {
            sb.replace(checkIdx, checkIdx + 2, "");
            checkIdx -= checkIdx != 0 ? 1 : 0;
            continue;
          }
        }
        // Next character
        checkIdx++;
      }
      return sb.toString();
  }
}

class MakeTheStringGreat {
  public static void main(String[] args) {
    Solution s = new Solution();
    MakeTheStringGreat.testMakeTheStringGreat(1, "leEeetcode", "leetcode", s);
    MakeTheStringGreat.testMakeTheStringGreat(2, "abBAcC", "", s);
    MakeTheStringGreat.testMakeTheStringGreat(3, "s", "s", s);
  }

  public static void testMakeTheStringGreat(int testNum, String input, String expected, Solution s) {
    String result = s.makeGood(input);

    System.out.println(
      String.format(
        "Test #%d:\n'%s' => '%s' / '%s'",
        testNum, input, result, expected
      )
    );
    assert(result.equals(expected));
  }
}