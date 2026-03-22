// Given a 0-indexed integer array nums of length n and an integer target, return the number of pairs (i, j) where 0 <= i < j < n and nums[i] + nums[j] < target.

struct Solution;
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut pairs = 0;
        for i in 0..n - 1 {
          for j in i + 1..n {
            if nums[i] + nums[j] < target {
              pairs += 1;
            }
          }
        }
        pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![-1, 1, 2, 3, 1];
        let target = 2;
        let expected = 3;
        assert_eq!(Solution::count_pairs(nums, target), expected);
    }

    #[test]
    fn test2() {
        let nums = vec![-6, 2, 5, -2, -7, -1, 3];
        let target = -2;
        let expected = 10;
        assert_eq!(Solution::count_pairs(nums, target), expected);
    }
}