// You are given a string s consisting of lowercase letters and an integer k. We call a string t ideal if the following conditions are satisfied:
  // t is a subsequence of the string s.
  // The absolute difference in the alphabet order of every two adjacent letters in t is less than or equal to k.
  // Return the length of the longest ideal string.
// A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.

// Note that the alphabet order is not cyclic. For example, the absolute difference in the alphabet order of 'a' and 'z' is 25, not 1.

public class LongestIdealString {
  public static void main(String[] args) {}

  public static void testLongestIdealString(int testNum, String input, int k, int expected, Solution s) {

  }
}

class Solution {
  public int longestIdealString(String s, int k) {
      int[] dp = new int[26];
      int res = 1;
      for (int idx = 0; idx < s.length(); idx++) {
        int elemIdx = s.charAt(idx) - 'a';
        for (int neighbor = elemIdx; neighbor >= 0 && neighbor >= elemIdx - k; neighbor--) {
          dp[elemIdx] = Math.max(dp[elemIdx], dp[neighbor] + 1);
        }
        for (int neighbor = elemIdx + 1; neighbor < 26 && neighbor <= elemIdx + k; neighbor++) {
          dp[elemIdx] = Math.max(dp[elemIdx], dp[neighbor] + 1);
        }
        res = Math.max(res, dp[elemIdx]);
      }
      return res;
  }
}