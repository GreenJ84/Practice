// Given an integer array of size n, find all elements that appear more than ⌊ n/3 ⌋ times.

//! Constraints:

// 1 <= nums.length <= 5 * 104
// -109 <= nums[i] <= 109


struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
      let k = (nums.len() / 3) as i32;
        let mut map = HashMap::<i32, i32>::new();

        for num in nums {
          map.entry(num)
            .and_modify(|count| { *count += 1 })
            .or_insert(1);
        }

        let mut ans = Vec::<i32>::new();
        for (num, count) in map.iter() {
          if count > &k {
            ans.push(*num);
          }
        }
        ans
    }
}