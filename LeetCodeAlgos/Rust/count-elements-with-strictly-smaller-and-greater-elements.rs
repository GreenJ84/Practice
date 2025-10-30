// Given an integer array nums, return the number of elements that have both a strictly smaller and a strictly greater element appear in nums.

struct Solution;
impl Solution {
    pub fn count_elements(nums: Vec<i32>) -> i32 {
      let mut smallest = (i32::MAX, 0);
      let mut largest = (i32::MIN, 0);
      for num in &nums {
        match num {
          n if n > &largest.0 => {
            largest = (*n, 1);
          },
          n if n == &largest.0 => {
            largest.1 += 1;
          },
          _ => {}
        }

        match num {
          n if n < &smallest.0 => {
            smallest = (*n, 1);
          },
          n if n == &smallest.0 => {
            smallest.1 += 1;
          },
          _ => {}
        }
      }

      if smallest.0.eq(&largest.0) {
        return (nums.len() as i32) - smallest.1;
      }
      (nums.len() as i32) - smallest.1 - largest.1
    }
}