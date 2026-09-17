// You are given an integer array nums.

// The digit range of an integer is defined as the difference between its largest digit and smallest digit.

// For example, the digit range of 5724 is 7 - 2 = 5.

// Return the sum of all integers in nums whose digit range is equal to the maximum digit range among all integers in the array.

// Constraints:
// 1 <= nums.length <= 100
// 10 <= nums[i] <= 105

struct Solution;
impl Solution {
    pub fn max_digit_range(nums: Vec<i32>) -> i32 {
        let (mut max_range, mut sum) = (0, 0);
        for i in 0..nums.len() {
            let mut n = nums[i];
            let (mut max, mut min) = (0, 10);
            while n > 0 {
                let digit = n % 10;
                max = max.max(digit);
                min = min.min(digit);
                n /= 10;
            }
            match (max_range, max - min) {
                (a, b) if a == b => {
                    sum += nums[i];
                }
                (a, b) if a < b => {
                    max_range = b;
                    sum = nums[i];
                }
                (_, _) => {}
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums1 = vec![5724, 111, 350];
        assert_eq!(Solution::max_digit_range(nums1), 6074);
    }
    #[test]
    fn test_2() {
        let nums2 = vec![90, 900];
        assert_eq!(Solution::max_digit_range(nums2), 990);
    }
}
