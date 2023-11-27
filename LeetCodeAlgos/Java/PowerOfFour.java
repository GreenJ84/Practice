// Given an integer n, return true if it is a power of four. Otherwise, return false.
// An integer n is a power of four, if there exists an integer x such that n == 4x.

class Solution {
    public boolean isPowerOfFour(int n) {
      return ( n > 0 && 
        (int) n == n && 
        Math.log(n) / Math.log(4) % 1 == 0
      );
    }
}

// class Solution {
//     public boolean isPowerOfFour(int n) {
//         if (n <= 0 || (int) n != n) return false;
//         double baseFour = Math.log(n) / Math.log(4);
//         return baseFour == (int) baseFour;
//     }
// }

class PowerOfFour {
  public static void main(String[] args) {
    Solution s = new Solution();
    boolean test1 = s.isPowerOfFour(16);
    System.out.println(test1);
    assert(test1);

    boolean test2 = s.isPowerOfFour(5);
    System.out.println(test2);
    assert(!test2);

    boolean test3 = s.isPowerOfFour(1);
    System.out.println(test3);
    assert(test3);
  }
}