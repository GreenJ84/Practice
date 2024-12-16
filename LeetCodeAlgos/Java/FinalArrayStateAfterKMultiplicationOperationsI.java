// You are given an integer array nums, an integer k, and an integer multiplier.

// You need to perform k operations on nums. In each operation:

// Find the minimum value x in nums. If there are multiple occurrences of the minimum value, select the one that appears first.
// Replace the selected minimum value x with x * multiplier.
// Return an integer array denoting the final state of nums after performing all k operations.

import java.util.*;

public class FinalArrayStateAfterKMultiplicationOperationsI {
  public static void main(String[] args) {

  }

  public int[] getFinalState(int[] nums, int k, int multiplier) {
      PriorityQueue<Map.Entry<Integer, Integer>> queue = new PriorityQueue<>(nums.length, (a, b) -> {
        if (a.getValue().equals(b.getValue())){
          return Integer.compare(a.getKey(), b.getKey());
        }
        return Integer.compare(a.getValue(), b.getValue());
      });

      for (int idx = 0; idx < nums.length; idx++) {
        queue.offer(Map.entry(idx, nums[idx]));
      }

      while (k > 0){
        Map.Entry<Integer, Integer> entry = queue.poll();

        int newValue = entry.getValue() * multiplier;
        queue.offer(Map.entry(entry.getKey(), newValue));
        k--;
      }

      while (!queue.isEmpty()) {
        Map.Entry<Integer, Integer> entry = queue.poll();
        nums[entry.getKey()] = entry.getValue();
      }
      return nums;
  }
}
