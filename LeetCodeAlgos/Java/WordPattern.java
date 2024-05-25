// Given a pattern and a string s, find if s follows the same pattern.

// Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in s.

import java.util.HashMap;

public class WordPattern {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testWordPattern(1, "abba", "dog cat cat dog", true, solution);
    testWordPattern(2, "abba", "dog cat cat fish", false, solution);
    testWordPattern(3, "aaaa", "dog cat cat dog", false, solution);
    testWordPattern(4, "abba", "dog dog dog dog", false, solution);
  }

  public static void testWordPattern(int testNum, String pattern, String str, boolean expected, Solution s) {
    boolean result = s.wordPattern(pattern, str);

    System.out.println(String.format(
      "Test %d: %b / %b (%s)",
      testNum,
      result,
      expected,
      result == expected ? "PASS" : "FAIL"
    ));
  }
}

class Solution {
  public boolean wordPattern(String pattern, String s) {
    String[] words = s.split("\\s");
    if (words.length!= pattern.length()) return false;
    HashMap<Character, String> charMap = new HashMap<Character, String>();

    for (int idx = 0; idx < pattern.length(); idx++) {
      Character current = pattern.charAt(idx);
      System.out.println(String.format("%c %s", current, words[idx]));
      boolean key = charMap.containsKey(current);
      boolean value = charMap.containsValue(words[idx]);
      if (!key && !value){
        charMap.put(current, words[idx]);
      } else if (!key && value || !charMap.get(current).equals(words[idx])) {
        return false;
      }
    }
    return true;
  }
}