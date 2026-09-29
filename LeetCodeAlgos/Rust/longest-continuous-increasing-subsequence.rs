// Given an unsorted array of integers nums, return the length of the longest continuous increasing subsequence (i.e. subarray). The subsequence must be strictly increasing.

// A continuous increasing subsequence is defined by two indices l and r (l < r) such that it is [nums[l], nums[l + 1], ..., nums[r - 1], nums[r]] and for each l <= i < r, nums[i] < nums[i + 1].

// Constraints:
// 1 <= nums.length <= 10^4
// -10^9 <= nums[i] <= 10^9

struct Solution;
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut longest = 1;
        let mut count = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                count += 1;
            } else {
                longest = longest.max(count);
                count = 1;
            }
        }
        longest.max(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 3, 5, 4, 7];
        assert_eq!(Solution::find_length_of_lcis(nums), 3);
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 2, 2, 2, 2];
        assert_eq!(Solution::find_length_of_lcis(nums), 1);
    }

    #[test]
    fn test_3() {
        let nums = vec![-4, 3, 1, 2, 4, 5];
        assert_eq!(Solution::find_length_of_lcis(nums), 4);
    }

    #[test]
    fn test_4() {
        let nums = vec![0, 3, 1, -100, 2, 4, 5];
        assert_eq!(Solution::find_length_of_lcis(nums), 4);
    }

    #[test]
    fn test_5() {
        let nums = vec![0, 3, 1, -100, -5, -1, 2, 4, 5];
        assert_eq!(Solution::find_length_of_lcis(nums), 6);
    }
}
