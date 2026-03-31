// Given an array nums, return true if the array was originally sorted in non-decreasing order, then rotated some number of positions (including zero). Otherwise, return false.

// There may be duplicates in the original array.

// Note: An array A rotated by x positions results in an array B of the same length such that B[i] == A[(i+x) % A.length] for every valid index i.

struct Solution;
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut count = if nums[n - 1] > nums[0] { 1 } else { 0 };
        for i in 1..n {
          if nums[i - 1] > nums[i] {
            if count == 1 { return false; }
            count += 1;
          }
        }
        return true;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![3, 4, 5, 1, 2];
        assert_eq!(Solution::check(nums), true);
    }

    #[test]
    fn test2() {
        let nums = vec![2, 1, 3, 4];
        assert_eq!(Solution::check(nums), false);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::check(nums), true);
    }
}
