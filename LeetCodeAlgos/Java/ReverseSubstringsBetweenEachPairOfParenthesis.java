// You are given a string s that consists of lower case English letters and brackets.

// Reverse the strings in each pair of matching parentheses, starting from the innermost one.

// Your result should not contain any brackets.

import java.util.*;

public class ReverseSubstringsBetweenEachPairOfParenthesis {
  public static void main(String[] args) {
    Solution s = new Solution();

    testReverseParentheses(1, "(abcd)", "dcba", s);
    testReverseParentheses(2, "(u(love)i)", "iloveu", s);
    testReverseParentheses(3, "(ed(et(oc))el)", "leetcode", s);
    testReverseParentheses(4, "a(bcdefghijkl(mno)p)q", "apmnolkjihgfedcbq", s);
  }

  public static void testReverseParentheses(int testNum, String s, String expected, Solution sol) {
    String result = sol.reverseParentheses(s);

    System.out.println(String.format(
      "Test %d: %s / %s (%s)",
      testNum,
      result,
      expected,
      result.equals(expected)? "PASS" : "FAIL"
    ));
  }
}

class Solution {
  public String reverseParentheses(String s) {
    StringBuilder sb = new StringBuilder();
    Stack<Stack<Character>> levels = new Stack<>();
    Stack<Character> current = null;

    for (char c : s.toCharArray()) {
      if (c == '(') {
        if (current != null){
          levels.push(current);
        }
        current = new Stack<>();
      }
      else if (c == ')') {
        if (levels.isEmpty()) {
          while (!current.isEmpty()) {
            sb.append(current.pop());
          }
          current = null;
        }
        else {
          Stack<Character> outer = levels.pop();
          while (!current.isEmpty()) {
            outer.add(current.pop());
          }
          current = outer;
        }
      }
      else if (current == null) {
        sb.append(c);
      }
      else {
        current.add(c);
      }
    }

    return sb.toString();
  }
}