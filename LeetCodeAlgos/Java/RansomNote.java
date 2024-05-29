// Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.

// Each letter in magazine can only be used once in ransomNote.

import java.util.HashMap;

public class RansomNote {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testCanConstruct(1, "a", "b", false, solution);
    testCanConstruct(2, "aa", "ab", false, solution);
    testCanConstruct(3, "aa", "aab", true, solution);
  }

  public static void testCanConstruct(int testNum, String ransomNote, String magazine, boolean expected, Solution s){
    boolean result = s.canConstruct(ransomNote, magazine);

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
  public boolean canConstruct(String ransomNote, String magazine) {
      HashMap<Character, Integer> map = new HashMap<>();
      for (int i = 0; i < magazine.length(); i++) {
        if (map.containsKey(magazine.charAt(i))) {
          map.put(magazine.charAt(i), map.get(magazine.charAt(i)) + 1);
        } else {
          map.put(magazine.charAt(i), 1);
        }
      }
      for (int i = 0; i < ransomNote.length(); i++) {
        if (map.containsKey(ransomNote.charAt(i))) {
          map.put(ransomNote.charAt(i), map.get(ransomNote.charAt(i)) - 1);
          if (map.get(ransomNote.charAt(i)) == 0) {
            map.remove(ransomNote.charAt(i));
          }
        } else {
          return false;
        }
      }
      return true;
  }
}