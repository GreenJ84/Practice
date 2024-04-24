// The Tribonacci sequence Tn is defined as follows: 
  // T0 = 0, T1 = 1, T2 = 1, and Tn+3 = Tn + Tn+1 + Tn+2 for n >= 0.
// Given n, return the value of Tn.

public class NthTribonacciNumber {
  public static void main(String[] args) {
    Solution s = new Solution();

    testTribonacci(1, 4, 4, s);
    testTribonacci(2, 25, 1389537, s);
    testTribonacci(3, 0, 0, s);
    testTribonacci(4, 3, 2, s);
  }
  
  public static void testTribonacci(int testNum, int n, int expected, Solution s) {
    int result = s.tribonacci(n);

    System.out.println(String.format("Test %d: %d -> %d[%d, %b]", testNum, n, expected, result, result == expected));
    assert (result == expected);
  }
  
}

class Solution {
  public int tribonacci(int n) {
      int[] trib = new int[]{0, 1, 1};
      if (n < 3) {
        return trib[n];
      }
      for (int i = 3; i <= n; i++) {
        trib[i % 3] = trib[0] + trib[1] + trib[2];
      }
      return trib[n % 3];
  }
}