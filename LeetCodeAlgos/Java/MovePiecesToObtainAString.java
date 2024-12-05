// You are given two strings start and target, both of length n. Each string consists only of the characters 'L', 'R', and '_' where:

// The characters 'L' and 'R' represent pieces, where a piece 'L' can move to the left only if there is a blank space directly to its left, and a piece 'R' can move to the right only if there is a blank space directly to its right.
// The character '_' represents a blank space that can be occupied by any of the 'L' or 'R' pieces.
// Return true if it is possible to obtain the string target by moving the pieces of the string start any number of times. Otherwise, return false.

public class MovePiecesToObtainAString {
  public static void main(String[] args){
    class Solution {
      public boolean canChange(String start, String target) {
          int n = start.length();
          int leftNeed = 0, rightCount = 0;
          for (int i = 0; i < n; i++) {
            char st = start.charAt(i), trg = target.charAt(i);
              if (st == 'R'){
                if (trg == 'L' || leftNeed < 0) return false;
                rightCount++;
                if (trg == 'R') rightCount--;
              }
              else if (st == 'L'){
                if (trg == 'R' || rightCount > 0) return false;
                else if (trg == 'L') leftNeed--;
                if (leftNeed == 0) return false;
                leftNeed++;
              }
              else if (trg == 'R'){
                if (rightCount == 0 || leftNeed < 0) return false;
                rightCount--;
              }
              else if (trg == 'L'){
                if (rightCount > 0) return false;
                leftNeed--;
              }
          }
          return rightCount == 0 && leftNeed == 0;
      }
    }

    Solution solution = new Solution();
    System.out.println(solution.canChange("_L__R__R_", "L______RR"));
    System.out.println(solution.canChange("R_L_", "__LR"));
    System.out.println(solution.canChange("_R", "R_"));
  }
}