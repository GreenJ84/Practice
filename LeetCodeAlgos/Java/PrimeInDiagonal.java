// You are given a 0-indexed two-dimensional integer array nums.
// Return the largest prime number that lies on at least one of the diagonals of nums. In case, no prime is present on any of the diagonals, return 0.
// Note that:
  // An integer is prime if it is greater than 1 and has no positive integer divisors other than 1 and itself.
  // An integer val is on one of the diagonals of nums if there exists an integer i for which nums[i][i] = val or an i for which nums[i][nums.length - i - 1] = val.

class Solution {
    public int diagonalPrime(int[][] nums) {
      int n = nums.length;
      int ans = 0;
      for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
          if (i == j || j == n - i - 1){
            if (isPrime(nums[i][j])) {
              ans = Math.max(ans, nums[i][j]);
            }
          }
        }
      }
      return ans;
    }

    boolean isPrime(int n) {
      if (n <= 1) {
        return false;
      }
      for (int i = 2; i * i <= n; i++) {
        if (n % i == 0) {
          return false;
        }
      }
      return true;
    }
}

class PrimeInDiagonal {
  public static void main(String[] args) {
    Solution s = new Solution();

    int[][] nums1 = {{1, 2, 3}, {5,6,7}, {9,10,11}};
    int test1 = s.diagonalPrime(nums1);
    System.out.println(test1);
    assert test1 == 11;

    int[][] nums2 = {{1, 2, 3}, {5,17,7}, {9,11,10}};
    int test2 = s.diagonalPrime(nums2);
    System.out.println(test2);
    assert test2 == 17;
  }
}