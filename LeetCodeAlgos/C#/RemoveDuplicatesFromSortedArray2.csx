// Given an integer array nums sorted in non-decreasing order, remove some duplicates in-place such that each unique element appears at most twice. The relative order of the elements should be kept the same.
// Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.
// Return k after placing the final result in the first k slots of nums.
// Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.

//! Custom Judge:

// The judge will test your solution with the following code:
// int[] nums = [...]; // Input array
// int[] expectedNums = [...]; // The expected answer with correct length

// int k = removeDuplicates(nums); // Calls your implementation

// assert k == expectedNums.length;
// for (int i = 0; i < k; i++) {
//     assert nums[i] == expectedNums[i];
// }
// If all assertions pass, then your solution will be accepted.

public class Solution {
  public int RemoveDuplicates(int[] nums) {
    Dictionary<int, int> dict = new Dictionary<int, int>();

    int validIdx = 0;
    for (int idx = 0; idx < nums.Length; idx++) {
      if (!dict.ContainsKey(nums[idx])) {
        dict.Add(nums[idx], 1);
      } else {
        dict[nums[idx]]++;
      }

      if (dict[nums[idx]] <= 2) {
        nums[validIdx++] = nums[idx];
      }
    }

    return validIdx;
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

TestRemoveDuplicates(1, [1,1,1,2,2,3], 5, [1,1,2,2,3,0]);
TestRemoveDuplicates(2, [0,0,1,1,1,1,2,3,3], 7, [0,0,1,1,2,3,3,0,0]);