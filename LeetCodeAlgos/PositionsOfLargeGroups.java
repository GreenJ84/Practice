// In a string s of lowercase letters, these letters form consecutive groups of the same character.
// For example, a string like s = "abbxxxxzyy" has the groups "a", "bb", "xxxx", "z", and "yy".
// A group is identified by an interval [start, end], where start and end denote the start and end indices (inclusive) of the group. In the above example, "xxxx" has the interval [3,6].
// A group is considered large if it has 3 or more characters.
// Return the intervals of every large group sorted in increasing order by start index.

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

class Solution {
    public List<List<Integer>> largeGroupPositions(String s) {
      List<List<Integer>> pos = new ArrayList<>();
      char curr = s.charAt(0);
      int start = 0;
      for (int i = 1; i < s.length(); i++) {
        if (s.charAt(i) != curr) {
          if (i - start >= 3){
            pos.add(Arrays.asList(start, i - 1));
          }
          curr = s.charAt(i);
          start = i;
        }
      }
      // Check the last group
        if (s.length() - start >= 3) {
            pos.add(Arrays.asList(start, s.length() - 1));
        }
      return pos;
    }
}

class PosistionsOfLargeGroups {
  public static void main(String[] args) {
    Solution s = new Solution();
    System.out.println(s.largeGroupPositions("abbxxxx"));
    System.out.println(s.largeGroupPositions("abc"));
    System.out.println(s.largeGroupPositions("abcdddeeeeaabbbcd"));
  }
}