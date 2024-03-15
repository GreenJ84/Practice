// Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].

// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

// You must write an algorithm that runs in O(n) time and without using the division operation.

public class Solution {
  public int[] ProductExceptSelf(int[] nums) {
      int[] result = new int[nums.Length];
      int idx = 0;
      int prod = 1;
      int zeros = 0;
      while (idx < nums.Length){
        if (nums[idx] == 0){ zeros++; }
        if (zeros > 1){
          return [.. result.Select(nums => 0)];
        }

        result[idx] = prod;
        prod *= nums[idx++];
      }
      prod = 1;
      while (idx > 0){
        result[--idx] *= prod;
        prod *= nums[idx];
      }
      return result;
  }
}

public void TestProductExceptSelf(int testNumber, int[] input, int[] expected){
  Solution solution = new Solution();

  int[] result = solution.ProductExceptSelf(input);
  bool passed = result.SequenceEqual(expected);

  Console.WriteLine($"Test {testNumber}: [{string.Join(",", input)}] => [{string.Join(",", result)}]/[{string.Join(",", expected)}]({passed})");
}

TestProductExceptSelf(1, [1,2,3,4], [24,12,8,6]);
TestProductExceptSelf(2, [-1,1,0,-3,3], [0,0,9,0,0]);
TestProductExceptSelf(3, [0,-1,1,0,-3,3,0], [0,0,0,0,0,0,0]);