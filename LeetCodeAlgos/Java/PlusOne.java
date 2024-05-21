
import java.util.*;

public class PlusOne {
  public static void main(String[] args) {
    Solution s = new Solution();

    testPlusOne(1, new int[]{1,2,3}, new int[]{1,2,4}, s);
    testPlusOne(2, new int[]{9}, new int[]{1,0}, s);
    testPlusOne(3, new int[]{1,9,9}, new int[]{2,0,0}, s);
    testPlusOne(4, new int[]{9,9,9}, new int[]{1,0,0,0}, s);
    testPlusOne(5, new int[]{1,0,0,0,0}, new int[]{1,0,0,0,1}, s);
  }
  
  public static void testPlusOne(int testNum, int[] digits, int[] expected, Solution s){
    int[] result = s.plusOne(digits);
    System.out.println(14 / 10);

    System.out.println(String.format(
      "Test %d: %s / %s (%s)",
      testNum,
      Arrays.toString(result),
      Arrays.toString(expected),
      Arrays.equals(result, expected) ? "PASS" : "FAIL"
    ));
  }
}

class Solution {
  public int[] plusOne(int[] digits) {
    int idx = digits.length - 1;
    int curr = digits[idx] + 1;
    while (curr >= 10 && idx > 0){
      System.out.print(Arrays.toString(digits) + " ");
      System.out.print(curr + " ");
      digits[idx] = curr % 10;
      curr = curr / 10 + digits[--idx];
      System.out.println(Arrays.toString(digits));
    }
    digits[idx] = curr % 10;
    curr /= 10;
    if (curr == 0){
      return digits;
    } else {
      int[] result = new int[digits.length + 1];
      result[0] = curr;
      for (int i = 0; i < digits.length; i++){
        result[i + 1] = digits[i];
      }
      return result;
    }
  }
}