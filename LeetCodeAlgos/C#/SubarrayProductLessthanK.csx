// Given an array of integers nums and an integer k, return the number of contiguous subarrays where the product of all the elements in the subarray is strictly less than k.


public class Solution {
    public int NumSubarrayProductLessThanK(int[] nums, int k) {
        int result = 0;
        for (int i = 0; i < nums.Length; i++) {
          int product = 1;
          for (int j = i; j < nums.Length; j++) {
            product *= nums[j];
            if (product >= k) {
              break;
            }
            
            result++;
          }
        }
        return result;
    }
};