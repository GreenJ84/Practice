// A string is a valid parentheses string (denoted VPS) if it meets one of the following:
  // It is an empty string "", or a single character not equal to "(" or ")",
  // It can be written as AB (A concatenated with B), where A and B are VPS's, or
  // It can be written as (A), where A is a VPS.
// We can similarly define the nesting depth depth(S) of any VPS S as follows:
  // depth("") = 0
  // depth(C) = 0, where C is a string with a single character not equal to "(" or ")".
  // depth(A + B) = max(depth(A), depth(B)), where A and B are VPS's.
  // depth("(" + A + ")") = 1 + depth(A), where A is a VPS.
// For example, "", "()()", and "()(()())" are VPS's (with nesting depths 0, 1, and 2), and ")(" and "(()" are not VPS's.

// Given a VPS represented as string s, return the nesting depth of s.

class Solution {
  public int maxDepth(String s) {
      int maxDepth = 0;
      int currDepth = 0;
      for (int i = 0; i < s.length(); i++) {
        switch (s.charAt(i)) {
          case '(':
            currDepth++;
            maxDepth = Math.max(maxDepth, currDepth);
            break;
          case ')':
            currDepth--;
            break;
        }
      }
      return maxDepth;
  }
}

class MaximumNestingDepthOfParenthesis {
  public static void main(String[] args) {
    Solution solution = new Solution();
    
    int ans1 = solution.maxDepth("(1+(2*3)+((8)/4))+1");
    System.out.println("" + ans1 + " / 3");
    assert ans1 == 3;

    int ans2 = solution.maxDepth("(1)+((2))+(((3)))");
    System.out.println("" + ans2 + " / 3");
    assert ans2 == 3;
  }
}