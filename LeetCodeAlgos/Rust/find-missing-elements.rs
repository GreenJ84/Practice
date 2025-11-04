// You are given an integer array nums consisting of unique integers.

// Originally, nums contained every integer within a certain range. However, some integers might have gone missing from the array.

// The smallest and largest integers of the original range are still present in nums.

// Return a sorted list of all the missing integers in this range. If no integers are missing, return an empty list.

struct Solution;
impl Solution {
    pub fn find_missing_elements(mut nums: Vec<i32>) -> Vec<i32> {
      nums.sort_unstable();

      let mut ans = Vec::<i32>::new();
      let mut expected = nums[0] + 1;

      for idx in 1..nums.len() - 1 {
        while nums[idx] != expected {
          ans.push(expected);
          expected += 1;
        }
        expected += 1;
      }
      ans
    }
}