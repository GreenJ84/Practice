import java.util.*;

public class LongestContinuousSubarrayWithAbsoluteDiffLTEToLimit {
  public static void main(String[] args) {
    Solution solution = new Solution();

    testLongestSubarray(1, new int[]{8,2,4,7}, 4, 2, solution);
    testLongestSubarray(2, new int[]{10,1,2,4,7,2}, 5, 4, solution);
    testLongestSubarray(3, new int[]{4,2,2,2,4,4,2,2}, 0, 3, solution);
  }

  private static void testLongestSubarray(int testNum, int[] nums, int limit, int expected, Solution s){
    int result = s.longestSubarray(nums, limit);

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
  class SortedWindow extends  ArrayList<Integer> {
    private int max = Integer.MIN_VALUE;
    private int min = Integer.MAX_VALUE;

    SortedWindow(){
      super();
    }

    public void sortAdd(Integer element){
      // Track max/min as added
      if (element > this.max) this.max = element;
      if (element < this.min) this.min = element;
      // Add element sorted
      if (this.size() == 0) this.add(element);
      else {
        int idx = 0;
        while (idx < this.size() && this.get(idx).compareTo(element) > 0){
          idx++;
        }
        this.add(idx, element);
      }
    }

    public void sortRemove(Integer element){
      // Track max/min as removed
      if (element == this.max)
        this.max = this.get(1);
      else if (element == this.min)
        this.min = this.get(this.size() - 2);
      this.remove((Object)element);

    }

    public int getDiff(){
      return Math.abs(this.max - this.min);
    }
  }

  public int longestSubarray(int[] nums, int limit) {
    SortedWindow window = new SortedWindow();
    int maxWindow = 0, left = 0;

    for (int right = 0; right < nums.length; right++){
      window.sortAdd(nums[right]);
      while (left < right && window.getDiff() > limit){
        window.sortRemove(nums[left]);
        left++;
      }
      if (window.getDiff() <= limit && right - left + 1 > maxWindow)
        maxWindow = right - left + 1;
    }
    return maxWindow;
  }
}