// Given two strings s and t, return true if t is an anagram of s, and false otherwise.

// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

import java.util.HashMap;
public class ValidAnagram {
  public static void main(String[] args) {
    Solution sol = new Solution();

    testIsAnagram(1, "anagram", "nagaram", true, sol);
    testIsAnagram(2, "rat", "car", false, sol);

    testIsAnagram(3, "a", "a", true, sol);
    testIsAnagram(4, "a", "b", false, sol);
    testIsAnagram(5, "a", "", false, sol);

  }

  public static void testIsAnagram(int testNum, String s, String t, boolean expected, Solution sol) {
    boolean result = sol.isAnagram(s, t);

    System.out.println(String.format(
      "Test %d: %b / %b (%s)",
      testNum,
      result,
      expected,
      result == expected? "PASS" : "FAIL"
    ));
  }
}

class Solution {
  public boolean isAnagram(String s, String t) {
    if (s.length()!= t.length()) return false;
    if (s.length() == 0 || s.equals(t)) return true;

    Character ch;
    HashMap<Character, Integer> map = new HashMap<Character, Integer>();
    int idx = 0;
    while (idx < s.length()) {
      ch = s.charAt(idx++);
      map.put(
        ch,
        (map.containsKey(ch) ? map.get(ch) : 0) + 1
      );
    }

    while (idx > 0){
      ch = t.charAt(--idx);
      if (!map.containsKey(ch)) return false;
      int allotted = map.get(ch).intValue();
      if (allotted < 1) return false;
      map.put(
        ch,
        allotted - 1
      );
    }

    return true;
  }
}