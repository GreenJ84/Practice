// Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.

// A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.

import java.util.*;

public class LetterCombinationsOfAPhoneNumber {
  public static void main(String[] args) {
    Solution s = new Solution();

    List<String> test1 = s.letterCombinations("23");
    System.out.println(test1);
  }
}

class Solution {
  private List<String> result = new ArrayList<>();
  private Map<Character, Character[]> map = Map.ofEntries(
    Map.entry('2', new Character[] {'a', 'b', 'c'}),
    Map.entry('3', new Character[] {'d', 'e', 'f'}),
    Map.entry('4', new Character[] {'g', 'h', 'i'}),
    Map.entry('5', new Character[] {'j', 'k', 'l'}),
    Map.entry('6', new Character[] {'m', 'n', 'o'}),
    Map.entry('7', new Character[] {'p', 'q', 'r', 's'}),
    Map.entry('8', new Character[] {'t', 'u', 'v'}),
    Map.entry('9', new Character[] {'w', 'x', 'y', 'z'})
  );
  public List<String> letterCombinations(String digits) {
    if (digits.length() == 0) {
      return result;
    }
    builder(digits, "", 0);
    return result;
  }

  private void builder(String digits, String combination, int index) {
    if (index == digits.length()) {
      result.add(combination);
      return;
    }
    Character[] letters = map.get(digits.charAt(index));
    for (Character letter : letters) {
      builder(digits, combination + letter, index + 1);
    }
  }
}