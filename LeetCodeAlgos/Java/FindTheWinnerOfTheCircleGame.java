// There are n friends that are playing a game. The friends are sitting in a circle and are numbered from 1 to n in clockwise order. More formally, moving clockwise from the ith friend brings you to the (i+1)th friend for 1 <= i < n, and moving clockwise from the nth friend brings you to the 1st friend.

// The rules of the game are as follows:

// Start at the 1st friend.
// Count the next k friends in the clockwise direction including the friend you started at. The counting wraps around the circle and may count some friends more than once.
// The last friend you counted leaves the circle and loses the game.
// If there is still more than one friend in the circle, go back to step 2 starting from the friend immediately clockwise of the friend who just lost and repeat.
// Else, the last friend in the circle wins the game.
// Given the number of friends, n, and an integer k, return the winner of the game.

import java.util.*;

public class FindTheWinnerOfTheCircleGame {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testFindTheWinner(1, 5, 2, 3, solution);
  }

  private static void testFindTheWinner(int testNum, int n, int k, int expected, Solution s){
    int result = s.findTheWinner(n, k);

    System.out.println(String.format(
      "Test %d: %d / %d (%b)",
      testNum,
      result,
      expected,
      result == expected
    ));
  }
}

class Solution {
  public int findTheWinner(int n, int k) {
      List<Integer> game = new ArrayList<Integer>();
      for (int i = 0; i < n; i++) {
        game.add(i);
      }
      int current = 0;
      for (int round = 0; round < n - 2; round++){
        current = (current + k - 1) % game.size();
        game.remove(current);
        if (current == game.size())
          current = 0;
      }
      return game.get(0) + 1;
  }
}