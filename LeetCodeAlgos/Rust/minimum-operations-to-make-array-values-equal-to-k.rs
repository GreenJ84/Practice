// You are given an integer array nums and an integer k.

// An integer h is called valid if all values in the array that are strictly greater than h are identical.

// For example, if nums = [10, 8, 10, 8], a valid integer is h = 9 because all nums[i] > 9 are equal to 10, but 5 is not a valid integer.

// You are allowed to perform the following operation on nums:

// Select an integer h that is valid for the current values in nums.
// For each index i where nums[i] > h, set nums[i] to h.
// Return the minimum number of operations required to make every element in nums equal to k. If it is impossible to make all elements equal to k, return -1.

struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
      let mut min = i32::MAX;
      let mut set = std::collections::HashSet::<i32>::new();
      for num in &nums {
        if num < &k {
          return -1;
        }
        if set.insert(*num) {
          if num < &min {
            min = *num;
          }
        }
      }
      set.len() as i32 - if min == k { 1 } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![5, 2, 5, 4, 5];
        let k = 2;
        assert_eq!(Solution::min_operations(nums, k), 2);
    }

    #[test]
    fn test2() {
        let nums = vec![2, 1, 2];
        let k = 2;
        assert_eq!(Solution::min_operations(nums, k), -1);
    }

    #[test]
    fn test3() {
        let nums = vec![9, 7, 5, 3];
        let k = 1;
        assert_eq!(Solution::min_operations(nums, k), 4);
    }
}