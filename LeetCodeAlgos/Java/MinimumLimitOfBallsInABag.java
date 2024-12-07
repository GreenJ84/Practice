// You are given an integer array nums where the ith bag contains nums[i] balls. You are also given an integer maxOperations.

// You can perform the following operation at most maxOperations times:

// Take any bag of balls and divide it into two new bags with a positive number of balls.
// For example, a bag of 5 balls can become two new bags of 1 and 4 balls, or two new bags of 2 and 3 balls.
// Your penalty is the maximum number of balls in a bag. You want to minimize your penalty after the operations.

// Return the minimum possible penalty after performing the operations.

import java.util.Arrays;

class MinimumLimitOfBallsInABag {
  public static void main(String[] args) {
    MinimumLimitOfBallsInABag obj = new MinimumLimitOfBallsInABag();

    System.out.println(obj.minimumSize(new int[]{9}, 2));
    System.out.println(obj.minimumSize(new int[]{2,4,8,2}, 4));
  }

  public int minimumSize(int[] nums, int maxOperations) {
    int left = 1, right = Arrays.stream(nums).max().getAsInt();
    while (left < right) {
      int mid = left + (right - left) / 2;
      if (canPartition(nums, mid, maxOperations)) {
        right = mid;
      } else {
        left = mid + 1;
      }
    }
    return left;
  }

  private boolean canPartition(int[] nums, int target, int maxOperations) {
    int ops = 0;
    for (int num : nums) {
      if (num > target){

        ops += (num - 1) / target;
        if (ops > maxOperations) {
          return false;
        }
      }
    }
    return ops <= maxOperations;
  }
}