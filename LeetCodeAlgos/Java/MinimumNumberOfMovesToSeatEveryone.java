// There are n seats and n students in a room. You are given an array seats of length n, where seats[i] is the position of the ith seat. You are also given the array students of length n, where students[j] is the position of the jth student.
// You may perform the following move any number of times:
  // Increase or decrease the position of the ith student by 1 (i.e., moving the ith student from position x to x + 1 or x - 1)
// Return the minimum number of moves required to move each student to a seat such that no two students are in the same seat.

// Note that there may be multiple seats or students in the same position at the beginning.

import java.util.*;

public class MinimumNumberOfMovesToSeatEveryone {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testMinMovesToSeat(1, new int[]{3,1,5}, new int[]{2,7,4}, 4, solution);

    testMinMovesToSeat(2, new int[]{4,1,5,9}, new int[]{1,3,2,6}, 7, solution);

    testMinMovesToSeat(3, new int[]{2,2,6,6}, new int[]{1,3,2,6}, 4, solution);
  }

  private static void testMinMovesToSeat(int testNum, int[] seats, int[] students, int expected, Solution s){
    int result = s.minMovesToSeat(seats, students);

    System.out.println(String.format(
      "Test %d: %d / %d (%s)",
      testNum,
      result,
      expected,
      result == expected? "PASS" : "FAIL"
    ));
  }
}

class Solution {
  public int minMovesToSeat(int[] seats, int[] students) {
    int res = 0;
    Arrays.sort(seats);
    Arrays.sort(students);
    for (int i = 0; i < seats.length; i++) {
      res += Math.abs(seats[i] - students[i]);
    }
    return res;
  }
}