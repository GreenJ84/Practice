// Given an array nums of size n, return the majority element.

// The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.

//! 0(n) time | O(1) space comes from Bitwise operations. IDK HOW though..

public class Solution {
    public int MajorityElement(int[] nums) {
        Dictionary<int, int> freq = new Dictionary<int, int>();
        foreach (int num in nums) {
          if (freq.ContainsKey(num)) {
            freq[num] += 1;
          } else {
            freq.Add(num, 1);
          }

          if (freq[num] >= nums.Length / 2 + 1) {
            return num;
          }
        }
        return -1;
    }
}

public void TestMajorityElement(int testNumber, int[] input, int expected){
  Solution solution = new Solution();

  int result = solution.MajorityElement(input);
  bool passed = result == expected;

  Console.WriteLine($"Test {testNumber}: [{string.Join(",", input)}] => {result} / {expected}({passed})");
}

TestMajorityElement(1, [3,2,3], 3);
TestMajorityElement(2, [2,2,1,1,1,2,2], 2);
TestMajorityElement(3, [1], 1);