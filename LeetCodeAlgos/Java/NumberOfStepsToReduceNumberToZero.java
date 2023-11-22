// NumberOfStepsToReduceNumberToZero
// Given an integer num, return the number of steps to reduce it to zero.

// In one step, if the current number is even, you have to divide it by 2, otherwise, you have to subtract 1 from it.

class Solution {
    public int numberOfSteps(int num) {
        int steps = 0;
        while (num > 0){
          if (num % 2 == 1){
            num -= 1;
          } else {
            num /= 2;
          }
          steps++;
        }
        return steps;
    }
}

class NumberOfStepsToReduceNumberToZero {
  public static void main(String[] args){
    Solution solution = new Solution();
    System.out.println(solution.numberOfSteps(14));
    System.out.println(solution.numberOfSteps(8));
    System.out.println(solution.numberOfSteps(123));
  }
}