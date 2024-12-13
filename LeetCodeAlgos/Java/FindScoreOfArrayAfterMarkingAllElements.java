// You are given an array nums consisting of positive integers.

// Starting with score = 0, apply the following algorithm:

// Choose the smallest integer of the array that is not marked. If there is a tie, choose the one with the smallest index.
// Add the value of the chosen integer to score.
// Mark the chosen element and its two adjacent elements if they exist.
// Repeat until all the array elements are marked.
// Return the score you get after applying the above algorithm.

import java.util.*;
import java.util.Map.Entry;

public class FindScoreOfArrayAfterMarkingAllElements {
  public static void main(String[] args) {
    FindScoreOfArrayAfterMarkingAllElements obj = new FindScoreOfArrayAfterMarkingAllElements();

    System.out.println(obj.findScore(new int[]{2,1,3,4,5,2})); // Output: 7
    System.out.println(obj.findScore(new int[]{2,3,5,1,3,2})); // Output: 5
    System.out.println(obj.findScore(new int[]{2,5,6,6,10})); // Output: 18
  }
  public long findScore(int[] nums) {
      Set<Integer> marked = new HashSet<>();
      PriorityQueue<Entry<Integer, Integer>> queue = new PriorityQueue<>((a, b) -> {
        int compare = Integer.compare(a.getKey(), b.getKey());
        if (compare == 0) {
            return Integer.compare(a.getValue(), b.getValue());
        }
        return compare;
      });
    
      long score = 0;

      for (int i = 0; i < nums.length; i++) {
          queue.offer(Map.entry(nums[i], i));
      }

      while (!queue.isEmpty()) {
        Entry<Integer, Integer> smallest = queue.poll();
        if (marked.contains(smallest.getValue())) {
            continue;
        }
        int num = smallest.getKey();
        int index = smallest.getValue();
        if (!marked.contains(index)) {
            score += num;
            marked.add(index);
            if (index - 1 >= 0 && !marked.contains(index - 1)) {
                marked.add(index - 1);
            }
            if (index + 1 < nums.length && !marked.contains(index + 1)) {
                marked.add(index + 1);
            }
        }
      }
      return score;
  }
}
