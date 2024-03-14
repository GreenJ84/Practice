// Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.

// Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things:

// Change the array nums such that the first k elements of nums contain the elements which are not equal to val. The remaining elements of nums are not important as well as the size of nums.
// Return k.

//! Custom Judge:
// The judge will test your solution with the following code:

// int[] nums = [...]; // Input array
// int val = ...; // Value to remove
// int[] expectedNums = [...]; // The expected answer with correct length.
// It is sorted with no values equaling val.

// int k = removeElement(nums, val); // Calls your implementation

// assert k == expectedNums.length;
// sort(nums, 0, k); // Sort the first k elements of nums
// for (int i = 0; i < actualLength; i++) {
//     assert nums[i] == expectedNums[i];
// }
// If all assertions pass, then your solution will be accepted.

public class Solution {
    public int RemoveElement(int[] nums, int val) {
      int switcher = nums.Length - 1;
      for (int idx = 0; idx < nums.Length; idx++) {
        if (nums[idx] == val){
          while (nums[switcher] == val && switcher > idx){
            nums[switcher] = 0;
            switcher--;
          }
          if (switcher < idx){
            break;
          }
          nums[idx] = nums[switcher];
          nums[switcher] = int.MaxValue;
          switcher--;
        }
      }
      return switcher + 1;
    }
}

public void TestRemoveElement(int testNum, int[] nums, int val, int expected, int[] expectedArr) {
  Solution solution = new Solution();

  int ans = solution.RemoveElement(nums, val);
  bool passed = ans == expected;
  Array.Sort(nums, 0, ans);
  Array.Sort(expectedArr, 0, ans);
  for (int i = 0; i < ans; i++) {
    if (!passed || nums[i] != expectedArr[i]) {
      passed = false;
      break;
    }
  }

  Console.WriteLine($"Test Number {testNum}: [{string.Join(",", nums)}] -> [{string.Join(",", expectedArr)}] | passed: {passed}");
}

TestRemoveElement(1, new int[] {3,2,2,3}, 3, 2, new int[] {2,2,0,0});
TestRemoveElement(2, new int[] {0,1,2,2,3,0,4,2}, 2, 5, new int[] {0,1,4,0,3,0,0,0});
TestRemoveElement(3, new int[] {1}, 1, 0, new int[] {});
TestRemoveElement(4, new int[] {4,5}, 5, 1, new int[] {4});
TestRemoveElement(5, new int[] {4,4,0,1,0,2}, 0, 4, new int[] {4,4,2,1});
TestRemoveElement(5, new int[] {2,2,2,0,0}, 0, 3, new int[] {2,2,2});