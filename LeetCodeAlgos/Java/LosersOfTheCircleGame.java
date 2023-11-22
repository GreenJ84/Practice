// There are n friends that are playing a game. The friends are sitting in a circle and are numbered from 1 to n in clockwise order. More formally, moving clockwise from the ith friend brings you to the (i+1)th friend for 1 <= i < n, and moving clockwise from the nth friend brings you to the 1st friend.
// The rules of the game are as follows:
  // 1st friend receives the ball.
  // After that, 1st friend passes it to the friend who is k steps away from them in the clockwise direction.
  // After that, the friend who receives the ball should pass it to the friend who is 2 * k steps away from them in the clockwise direction.
  // After that, the friend who receives the ball should pass it to the friend who is 3 * k steps away from them in the clockwise direction, and so on and so forth.
  // In other words, on the ith turn, the friend holding the ball should pass it to the friend who is i * k steps away from them in the clockwise direction.
// The game is finished when some friend receives the ball for the second time.
// The losers of the game are friends who did not receive the ball in the entire game.
//? Given the number of friends, n, and an integer k, return the array answer, which contains the losers of the game in the ascending order.

import java.util.*;

class Solution {
    public int[] circularGameLosers(int n, int k) {
        // If k == n, first jump returns to sender (all but 1)
        if (k == n) { 
          int[] ans = new int[n-1];
          for (int i = 0; i <= n - 2; i++) { 
            ans[i] = i+2; 
          }
          return ans;
        }
        // Save each current to be updated
        int curr = 1;
        // Save all positions that recieved the ball
        ArrayList<Integer> seen = new ArrayList<Integer>();
        // Loop until someone gets a second time
        while (!seen.contains(curr)) {
          System.out.println(curr);
          seen.add(curr);
          curr = (curr + k * seen.size()) % n;
          if (curr == 0){
            curr = n;
          }
        }

        int[] ans = new int[n - seen.size()];
        curr = 0;
        for (int i = 1; i <= n; i++) {
          if (!seen.contains(i)) {
            ans[curr++] = i;
          }
        }
        return ans;
    }
}

class LosersOfTheCircleGame {
  public static void main(String[] args){
    Solution s = new Solution();
    viewLosers(s.circularGameLosers(5, 2));
    viewLosers(s.circularGameLosers(4, 4));
    viewLosers(s.circularGameLosers(4, 1));
  }

  public static void viewLosers(int[] nums){
        System.out.print("[");
        for (int i = 0; i < nums.length; i++) {
            System.out.print(nums[i]);
            if (i < nums.length - 1) {
              System.out.print(", ");
            }
        }
        System.out.print("]");
        System.out.println();
        System.out.println("-----------------");
  }
}