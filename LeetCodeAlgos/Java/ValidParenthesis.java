// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

// An input string is valid if:

// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Every close bracket has a corresponding open bracket of the same type.

import java.util.*;

public class ValidParenthesis {
  public static void main(String[] args) {
    Solution sol = new Solution();

    testIsValid(1, "()", true, sol);
    testIsValid(2, "()[]{}", true, sol);
    testIsValid(3, "(]", false, sol);
    testIsValid(4, "([)]", false, sol);
    testIsValid(5, "{[]}", true, sol);
    testIsValid(6, "", true, sol);
    testIsValid(7, "([", false, sol);
    testIsValid(8, "([)]", false, sol);
  }

  static void testIsValid(int testNum, String s, Boolean expected, Solution sol) {
    boolean actual = sol.isValid(s);

    System.out.println(
      String.format(
        "Test #%d: '%s' => %b / %b",
        testNum,
        s,
        actual,
        expected
      )
    );
  }
}

class Solution {
  public boolean isValid(String s) throws IllegalArgumentException {
      Stack<Character> stack = new Stack<>();
      Map<Character, Character> map = new HashMap<>();
      map.put(')', '(');
      map.put('}', '{');
      map.put(']', '[');

      for (int i = 0; i < s.length(); i++) {
        switch (s.charAt(i)) {
          case '(':
          case '{':
          case '[':
            stack.push(s.charAt(i));
            break;
          case ')':
          case '}':
          case ']':
            if (stack.isEmpty() || map.get(s.charAt(i)) != stack.pop()) {
              return false;
            }
            break;
          default:
            throw new IllegalArgumentException();
        }
      }
      return stack.isEmpty();
  }
}