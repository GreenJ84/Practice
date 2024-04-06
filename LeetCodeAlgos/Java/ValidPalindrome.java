// A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.

// Given a string s, return true if it is a palindrome, or false otherwise.

class Solution {
  public boolean isPalindrome(String s) {
      int left = 0;
      int right = s.length() - 1;
      while (left < right) {
        while (!Character.isLetterOrDigit(s.charAt(left)) && left < right) {
          left++;

        }
        while (!Character.isLetterOrDigit(s.charAt(right)) && right > left) {
          right--;
        }
        char leftChar = s.charAt(left);
        char rightChar = s.charAt(right);
        // Case sensitivity conversions
        if (Character.isUpperCase(leftChar)) {
          leftChar = Character.toLowerCase(leftChar);
        }
        if (Character.isUpperCase(rightChar)) {

          rightChar = Character.toLowerCase(rightChar);
        }
        // Equality check for valid next chars
        if (leftChar!= rightChar) {
          return false;
        }
        // Close the pointers in
        left++;
        right--;
      }
      return true;
  }
}

public class ValidPalindrome {
  public static void main(String[] args) {
    Solution s = new Solution();
    ValidPalindrome.testIsPalindrome(1, "A man, a plan, a canal: Panama", true, s);
    ValidPalindrome.testIsPalindrome(2, "race a car", true, s);
    ValidPalindrome.testIsPalindrome(3, "", true, s);
    ValidPalindrome.testIsPalindrome(4, ".,", true, s);
  }

  public static void testIsPalindrome(int testNum, String input, Boolean expected, Solution s) {
    Boolean result = s.isPalindrome(input);

    System.out.println(
      String.format(
        "Test #%d:\n'%s' => %b / %b",
        testNum, input, result, expected
      )
    );
  }
}
