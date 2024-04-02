class Solution {
    public int singleNumber(int[] nums) {
        int ans = 0;
      for (int num: nums){
        ans ^= num;
      }
      return ans;
    }
}

class singleNumber {
  public static void main(String[] args) {
    Solution s = new Solution();
    System.out.println(s.singleNumber(new int[]{1,2,2}));
    System.out.println(s.singleNumber(new int[]{4,1,2,1,2}));
    System.out.println(s.singleNumber(new int[]{1}));
  }
}