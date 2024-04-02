import java.util.ArrayList;

class smallestNumberFromTwoDigitArrays {
  public static void main(String[] args){
    Solution s = new Solution();
    System.out.println(s.minNumber(new int[]{4,1,3}, new int[]{5,7}));
    System.out.println(s.minNumber(new int[]{3,5,2,6}, new int[]{3,1,7}));
  }
}

class Solution {
    public int minNumber(int[] nums1, int[] nums2) {
      ArrayList<Integer> nums1List = new ArrayList<Integer>();
      // Smallest first array value
      int fMin = Integer.MAX_VALUE;
      // Iterate through the first array values
      for (int num: nums1) {
        nums1List.add(num);
        if (num < fMin) {
          fMin = num;
        }
      }

      // Smallest second array value
      int sMin = Integer.MAX_VALUE;
      // If arrays share a value (smallest number)
      boolean shared = false;
      // Iterate through the second array values
      for (int num: nums2) {
        if (nums1List.contains(num)) {
          // If the shared is the first minimum value, return it
          if (num == fMin){
            return fMin;
          }
          // Reset the fMin if a shared is found
          if (!shared){
            fMin = num;
          }
          shared = true;
          // If the shared is not first minimum value, replace fMin
          if (shared && num < fMin) {
            fMin = num;
          }
        }
        if (num < sMin) {
          sMin = num;
        }
      }

      if (shared) {
        return fMin;
      }
      else if (fMin < sMin) {
        return Integer.parseInt(Integer.toString(fMin).concat(Integer.toString(sMin)));
      }
      else {
        return Integer.parseInt(Integer.toString(sMin).concat(Integer.toString(fMin)));
      }
    }
}