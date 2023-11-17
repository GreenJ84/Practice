// Given an array nums sorted in non-decreasing order, return the maximum between the number of positive integers and the number of negative integers.

// In other words, if the number of positive integers in nums is pos and the number of negative integers is neg, then return the maximum of pos and neg.
// Note that 0 is neither positive nor negative.

class Solution {
    public int maximumCount(int[] nums) {
        int pos = 0;
        int neg = 0;
        for (int i = 0; i < nums.length; i++) {
          if (nums[i] > 0) {
            pos++;
          } 
          else if (nums[i] < 0){
            neg++;
          }
        }
        return Math.max(pos, neg);
    }
}

class MaximumCountOfPositiveNegativeIntegers {
  public static void main(String[] args){
    Solution s = new Solution();
    int test1 = s.maximumCount(new int[]{-2,-1,-1,1,2,3});
    System.out.println(test1);
    assert test1 == 3;

    int test2 = s.maximumCount(new int[]{-3,-2,-1,0,0,1,2});
    System.out.println(test2);
    assert test2 == 3;

    int test3 = s.maximumCount(new int[]{5,20,66,1314});
    System.out.println(test3);
    assert test3 == 2;

    int test4 = s.maximumCount(new int[]{});
    System.out.println(test4);
    assert test4 == 0;
  }
}