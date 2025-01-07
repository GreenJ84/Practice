// You have n boxes. You are given a binary string boxes of length n, where boxes[i] is '0' if the ith box is empty, and '1' if it contains one ball.

// In one operation, you can move one ball from a box to an adjacent box. Box i is adjacent to box j if abs(i - j) == 1. Note that after doing so, there may be more than one ball in some boxes.

// Return an array answer of size n, where answer[i] is the minimum number of operations needed to move all the balls to the ith box.

// Each answer[i] is calculated considering the initial state of the boxes.

import java.util.*;

public class MinimumNumberOfOperationsToMoveAllBallsToEachBox {
  public static void main(String[] args) {
    MinimumNumberOfOperationsToMoveAllBallsToEachBox obj = new MinimumNumberOfOperationsToMoveAllBallsToEachBox();

    System.out.println(Arrays.toString(obj.minOperations("110")));
    System.out.println(Arrays.toString(obj.minOperations("001011")));
    System.out.println(Arrays.toString(obj.minOperations("0")));
    System.out.println(Arrays.toString(obj.minOperations("1")));
    System.out.println(Arrays.toString(obj.minOperations("000000000")));
  }

  public int[] minOperations(String boxes) {
    int n = boxes.length();
    if (n == 1) return new int[]{0};
    int[] ans = new int[n];
    Arrays.fill(ans, 0);

    int balls = 0;
    for (int i = 1; i < n; i++){
      int ch = Character.getNumericValue(boxes.charAt(i - 1));
      balls += ch;
      ans[i] += balls + ans[i - 1];
    }

    int moves = 0;
    balls = 0;
    for (int i = n - 2; i >= 0; i--) {
      int ch = Character.getNumericValue(boxes.charAt(i + 1));
      balls += ch;
      ans[i] += balls + moves;
      moves += balls;
    }

    return ans;
  }
}
