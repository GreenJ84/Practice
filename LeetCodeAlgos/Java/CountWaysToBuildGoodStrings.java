
// Given the integers zero, one, low, and high, we can construct a string by starting with an empty string, and then at each step perform either of the following:

// Append the character '0' zero times.
// Append the character '1' one times.
// This can be performed any number of times.

// A good string is a string constructed by the above process having a length between low and high (inclusive).

// Return the number of different good strings that can be constructed satisfying these properties. Since the answer can be large, return it modulo 109 + 7.

public class CountWaysToBuildGoodStrings {
  public static void main(String[] args) {
    CountWaysToBuildGoodStrings obj = new CountWaysToBuildGoodStrings();

    System.out.println(obj.countGoodStrings(3, 3, 1, 1));
    System.out.println(obj.countGoodStrings(2, 3, 1, 2));
  }

  public int countGoodStrings(int low, int high, int zero, int one) {
    int MOD = 1_000_000_007;

    int[] dp = new int[high + 1];
    dp[0] = 1;

    for (int i = 1; i <= high; i++) {
        if (i >= zero) dp[i] = (dp[i] + dp[i - zero]) % MOD;
        if (i >= one) dp[i] = (dp[i] + dp[i - one]) % MOD;
    }

    int result = 0;
    for (int i = low; i <= high; i++) {
        result = (result + dp[i]) % MOD;
    }

    return result;
  }

  // int low, high;
  // String zero, one;
  // public int countGoodStrings(int low, int high, int zero, int one) {
  //   this.low = low;
  //   this.high = high;
  //   this.zero = String.valueOf('0').repeat(zero);
  //   this.one  = String.valueOf('1').repeat(one);

  //   return checkCombinations("");
  // }

  // public int checkCombinations(String curr){
  //   int n = curr.length();
  //   if (n > high) return 0;
  //   boolean good = curr.length() >= this.low && curr.length() <= this.high;
  //   return checkCombinations(curr + this.zero) +
  //     checkCombinations(curr + this.one) +
  //     (good ? 1 : 0);
  // }
}