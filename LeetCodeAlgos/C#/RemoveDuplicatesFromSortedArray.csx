// Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.

// Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:

// Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.
// Return k.

public class Solution {
    public int RemoveDuplicates(int[] nums) {
        int validIdx = 0;
        for(int idx = 1; idx < nums.Length; idx++){
          if (nums[idx] != nums[idx-1]){
            nums[++validIdx] = nums[idx];
          }
        }
        return ++validIdx;
    }
}

public void TestRemoveDuplicates(int testNumber, int[] input, int expected, int[] expectedArr){
  Solution solution = new Solution();
  int[] clone = (int[])input.Clone();

  int result = solution.RemoveDuplicates(input);
  bool passed = result == expected;
  for(int run = 0; run < result; run++){
    if (input[run] != expectedArr[run] || !passed){
      passed = false;
      break;
    }
  }

  Console.WriteLine($"Test {testNumber}: [{string.Join(",", clone)}] => {result}\n\t[{string.Join(",", input)}] / [{string.Join(",", expectedArr)}] ({passed})");

}

TestRemoveDuplicates(1, [1,1,2], 2, [1,2,0]);
TestRemoveDuplicates(2, [0,0,1,1,1,2,2,3,3,4], 5, [0,1,2,3,4,0,0,0,0,0]);
TestRemoveDuplicates(3, [1,2,3], 3, [1,2,3]);