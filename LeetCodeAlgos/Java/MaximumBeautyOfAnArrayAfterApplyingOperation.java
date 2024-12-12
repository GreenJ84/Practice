import java.util.Arrays;

public class MaximumBeautyOfAnArrayAfterApplyingOperation {
  public static void main(String[] args) {
    MaximumBeautyOfAnArrayAfterApplyingOperation obj = new MaximumBeautyOfAnArrayAfterApplyingOperation();

    System.out.println(obj.maximumBeauty(new int[]{4,6,1,2}, 2));
    System.out.println(obj.maximumBeauty(new int[]{1,1,1,1}, 4));
  }
  public int maximumBeauty(int[] nums, int k) {
      Arrays.sort(nums);
      int max = 1, left = 0;

      for (int right = 1; right < nums.length; right ++){
        if (nums[right] - nums[left] > 2 * k) left++;
        max = Math.max(max, right - left + 1);
      }
      return max;
  }
}
