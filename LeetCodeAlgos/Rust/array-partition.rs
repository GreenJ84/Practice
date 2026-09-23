// Given an integer array nums of 2n integers, group these integers into n pairs (a1, b1), (a2, b2), ..., (an, bn) such that the sum of min(ai, bi) for all i is maximized. Return the maximized sum.

// Constraints:
// 1 <= n <= 10^4
// nums.length == 2 * n
// -104 <= nums[i] <= 10^4

struct Solution;
impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.chunks(2).map(|pair| pair[0]).sum()
    }

    pub fn array_pair_sum1(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        (0..nums.len() / 2).map(|i| nums[2 * i]).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 4, 3, 2];
        assert_eq!(Solution::array_pair_sum(nums), 4);
    }

    #[test]
    fn test_2() {
        let nums = vec![6, 2, 6, 5, 1, 2];
        assert_eq!(Solution::array_pair_sum(nums), 9);
    }
}
