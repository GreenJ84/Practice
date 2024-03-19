// Given an integer array nums, rotate the array to the right by k steps, where k is non-negative.

public class Solution {
    public void Rotate(int[] nums, int k) {
        k = k > nums.Length ? k % nums.Length : k;
        int[] temp = new int[nums.Length];
        for (int i = 0; i < nums.Length; i++) {
            temp[(i + k) % nums.Length] = nums[i];
        }
        for (int i = 0; i < nums.Length; i++) {
            nums[i] = temp[i];
        }
    }
}

public void TestRotate(int testNum, int[] nums, int k, int[] expected){
  Solution sol = new Solution();

  int[] copy = (int[])nums.Clone();
  sol.Rotate(nums, k);

  bool passed = true;
  for (int idx = 0; idx < expected.Length; idx++) {
    if (nums[idx] != expected[idx]) {
      passed = false;
      break;
    }
  }

  Console.WriteLine($"Test {testNum}:  [{string.Join(",", copy)}] => [{string.Join(",", nums)}] / [{string.Join(",", expected)}] ({passed})");
}

TestRotate(1, new int[] {1,2,3,4,5,6,7}, 3, new int[] {5,6,7,1,2,3,4});
TestRotate(2, new int[] {-1,-100,3,99}, 2, new int[] {3,99,-1,-100});