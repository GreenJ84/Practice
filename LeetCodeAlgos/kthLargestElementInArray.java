import java.util.*;

class Solution {
    public int findKthLargest(int[] nums, int k) {
      PriorityQueue<Integer> pq = new PriorityQueue<>(k);
      for (int i = 0; i < k; i++) {
        pq.add(nums[i]);
      }
      for (int i = k; i < nums.length; i++) {
        if (nums[i] > pq.peek()) {
          pq.poll();
          pq.add(nums[i]);
        }
      }
      return pq.peek();
    }
}

class kthLargestElementsInArray {
  public static void main(String[] args) {
    Solution s = new Solution();
    int[] nums = {3,2,1,5,6,4};
    int k = 2;
    System.out.println(s.findKthLargest(nums, k));

    nums = new int[]{3,2,3,1,2,4,5,5,6};
    k = 4;
    System.out.println(s.findKthLargest(nums, k));
  }
}