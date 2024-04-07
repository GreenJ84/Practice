// Given a string s of '(' , ')' and lowercase English characters.

// Your task is to remove the minimum number of parentheses ( '(' or ')', in any positions ) so that the resulting parentheses string is valid and return any valid string.

// Formally, a parentheses string is valid if and only if:

// It is the empty string, contains only lowercase characters, or
// It can be written as AB (A concatenated with B), where A and B are valid strings, or
// It can be written as (A), where A is a valid string.
import java.util.Stack;

class Solution {
  public String minRemoveToMakeValid(String s) {
      Stack<Integer> st = new Stack<Integer>();
      StringBuilder sb = new StringBuilder(s);
      Integer runner = 0;

      while (runner < sb.length()) {
        // Open parenthesis
        if (sb.charAt(runner) == '(') {
          st.push(runner);
        }
        // Closed Parenthesis
        else if (sb.charAt(runner) == ')') {
          // Matching close
          if (!st.empty()) {
            st.pop();
          }
          // Non matching close - remove
          else {
            sb.delete(runner, runner + 1);
            runner -= 1;
          }
        }
        System.out.println(sb.toString());
        runner++;
      }
      // All unmatched opens - remove
      while (!st.empty()) {
        int index = st.pop();
        sb.delete(index, index + 1);
        System.out.println(sb.toString());
      }
      return sb.toString();
  }
}

public class MinimumRemoveToMakeValidParenthesis {
  public static void main(String[] args){
    Solution s = new Solution();

    MinimumRemoveToMakeValidParenthesis.testMinRemoveToMakeValid(1, "lee(t(c)o)de)", "lee(t(c)o)de", s);
    MinimumRemoveToMakeValidParenthesis.testMinRemoveToMakeValid(2, "a)b(c)d", "ab(c)d", s);
    MinimumRemoveToMakeValidParenthesis.testMinRemoveToMakeValid(3, "))((", "", s);
  }

  public static void testMinRemoveToMakeValid(int testNum, String input, String expected, Solution s){
    String result = s.minRemoveToMakeValid(input);

    System.out.println(
      String.format(
        "Test #%d: '%s' \n - '%s' / '%s' (%b)",
        testNum, input, result, expected, result.equals(expected)
      )
    );
  }
}

