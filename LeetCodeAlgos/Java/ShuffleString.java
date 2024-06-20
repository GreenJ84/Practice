// You are given a string s and an integer array indices of the same length. The string s will be shuffled such that the character at the ith position moves to indices[i] in the shuffled string.

// Return the shuffled string.

public class ShuffleString {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testRestoreString(1, "codeleet", new int[]{4,5,6,7,0,2,1,3}, "leetcode", solution);

    testRestoreString(2, "abc", new int[]{0,1,2}, "abc", solution);
  }

  private static void testRestoreString(int testNum, String str, int[] indices, String expected, Solution s) {
    String result = s.restoreString(str, indices);

    System.out.println(String.format(
      "Test %d: %s / %s (%b)",
      testNum,
      result,
      expected,
      result.equals(expected)
    ));
  }
}

class Solution {
    public String restoreString(String s, int[] indices) {
        char[] res = new char[s.length()];
        for(int idx = 0; idx < s.length(); idx++){
            res[indices[idx]] = s.charAt(idx);
        }
        return new String(res);
    }
}